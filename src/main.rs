use std::process;

use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use log::{error, info};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

mod models;

pub struct AppState {
    db: PgPool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL must be set");
        process::exit(1);
    });

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to Database");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {err}");
            process::exit(1);
        }
    };

    sqlx::migrate!().run(&pool).await.unwrap_or_else(|err| {
        error!("Failed to run database migrations: {err}");
        process::exit(1);
    });

    let address = std::env::var("DATAPORT_ADDRESS").unwrap_or("127.0.0.1".to_string());

    let port = std::env::var("DATAPORT_PORT").map(|val| {
        val.parse::<u16>().unwrap_or_else(|_| {
            error!("DATAPORT_PORT must be a number");
            process::exit(1);
        })
    }).unwrap_or(8080_u16);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind((address, port))?
        .run()
        .await
}
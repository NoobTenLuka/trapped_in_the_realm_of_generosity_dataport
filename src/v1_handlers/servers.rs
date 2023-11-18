use crate::AppState;
use actix_web::{post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AdminDatacenterRegistrationRequest {
    name: String,
    region: String,
}

#[post("")]
async fn register_datacenter(
    state: web::Data<AppState>,
    data: web::Json<AdminDatacenterRegistrationRequest>,
) -> impl Responder {
    let rec = sqlx::query!(
        r#"INSERT INTO datacenter (name, region) VALUES ($1, $2) RETURNING (id)"#,
        data.name,
        data.region
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    rec.id.to_string()
}

#[derive(Deserialize)]
struct AdminGameServerRegistrationRequest {
    name: String,
}

#[post("/game_servers")]
async fn register_game_server(
    state: web::Data<AppState>,
    path: web::Path<i16>,
    data: web::Json<AdminGameServerRegistrationRequest>,
) -> impl Responder {
    let datacenter_id = path.into_inner();
    let rec = sqlx::query!(
        r#"INSERT INTO game_server (name, datacenter_id) VALUES ($1, $2) RETURNING (id)"#,
        data.name,
        datacenter_id
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    rec.id.to_string()
}

pub fn datacenter_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/datacenters")
            .configure(game_server_api_config)
            .service(register_datacenter),
    );
}

fn game_server_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/{datacenter_id}").service(register_game_server));
}

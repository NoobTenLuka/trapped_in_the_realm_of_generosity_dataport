use actix_web::{get, HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::models::Character;
#[allow(unused_imports)]
use crate::models::Location;

#[derive(Serialize)]
struct AdminCharactersResponse {
    characters: Vec<Character>,
}

#[get("")]
async fn get_characters(app_state: web::Data<AppState>) -> impl Responder {
    let characters = sqlx::query_as!(
        Character,
        r#"SELECT id,
        name,
        digidollar,
        game_server_id,
        area_id,
        location as "location: Location",
        instance_id,
        instance_location as "instance_location: _",
        keycloak_user_id,
        creation_date,
        playtime
        FROM character"#
    )
        .fetch_all(&app_state.db)
        .await
        .unwrap();

    web::Json(AdminCharactersResponse { characters })
}

#[get("/{id}")]
async fn get_character_by_id(
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();

    let character = sqlx::query_as!(
        Character,
        r#"SELECT id,
        name,
        digidollar,
        game_server_id,
        area_id,
        location as "location: Location",
        instance_id,
        instance_location as "instance_location: _",
        keycloak_user_id,
        creation_date,
        playtime
        FROM character
        WHERE id = $1"#,
        id
    )
        .fetch_optional(&app_state.db)
        .await
        .unwrap();

    match character {
        Some(character) => HttpResponse::Ok().json(character),
        None => HttpResponse::NotFound().body("Character not found!"),
    }
}

#[derive(Deserialize)]
struct AdminCharacterCreationRequest {
    name: String,
    game_server_id: i16,
    area_id: i16,
    location: Location,
    keycloak_user_id: Uuid,
}

#[post("")]
async fn create_character(
    app_state: web::Data<AppState>,
    admin_character_creation_request: web::Json<AdminCharacterCreationRequest>,
) -> impl Responder {
    let rec = sqlx::query!(
        "INSERT INTO character (name, game_server_id, area_id, location, keycloak_user_id) VALUES ($1, $2, $3, $4, $5) RETURNING (id)",
        admin_character_creation_request.name,
        admin_character_creation_request.game_server_id,
        admin_character_creation_request.area_id,
        admin_character_creation_request.location as Location,
        admin_character_creation_request.keycloak_user_id
    ).fetch_one(&app_state.db).await.unwrap();

    rec.id.to_string()
}

pub fn character_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/characters")
            .service(get_characters)
            .service(get_character_by_id)
            .service(create_character),
    );
}

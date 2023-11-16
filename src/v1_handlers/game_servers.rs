use actix_web::web;

async fn register_game_server() {}

pub fn character_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/game_servers")
    );
}
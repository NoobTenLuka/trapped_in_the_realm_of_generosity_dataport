use actix_web::web;

use crate::v1_handlers::characters::character_api_config;

mod characters;
mod game_servers;

/// Adds the v1 API handlers to the given ServiceConfig with the /v1 scope
pub fn v1_handler_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1").configure(character_api_config)
    );
}
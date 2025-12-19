use crate::api::user as user_api;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/get-all", web::get().to(user_api::get_all))
            .route("/create", web::post().to(user_api::create)),
    );
}

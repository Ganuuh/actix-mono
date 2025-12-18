use actix_web::web;
pub mod user_route;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(user_route::configure));
}

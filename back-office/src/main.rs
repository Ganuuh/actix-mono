use actix_web::{App, HttpServer, web};

use crate::route::user_route;
use shared::db::connection::create_pool;

pub mod api;
pub mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_route::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

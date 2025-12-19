use actix_web::{App, HttpServer, web};

use crate::route::routes;
use shared::db::connection::create_pool;

pub mod api;
pub mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool();
    dbg!("Starting Back Office server on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

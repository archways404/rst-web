use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::routes::route1::hello;
use crate::routes::route2::{echo, manual_hello};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
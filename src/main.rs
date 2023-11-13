use actix_web::{App, HttpServer, Responder};
use std::env;
use crate::routes::{healthz, ping};

mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").expect("Missing port number");
    let address = env::var("ADDRESS").expect("Missing port number");
    let port = port.parse::<u16>().expect("Invalid port given");

    HttpServer::new(|| {
        App::new()
            .service(healthz)
            .service(ping)
    })
        .bind((address, port))?
        .run()
        .await
}
use actix_web::{get, HttpResponse, Responder};


#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("Alive")
}

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use std::env;
use crate::routes::{healthz, ping};

use mysql::prelude::*;

extern crate mysql;

mod routes;


#[get("/show")]
async fn showTables() -> HttpResponse {


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&database_url).unwrap());
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();

    let mut conn = pool.get_conn().expect("Failed to get connection from pool");


    // Run the SHOW TABLES query
    let tables: Vec<String> = conn
        .query_map("SHOW TABLES", |row: mysql::Row| {
            let table_name: String = mysql::from_row(row);
            table_name
        })
        .expect("Failed to execute query");

    HttpResponse::Ok().body(tables.join(","))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = port.parse::<u16>().expect("Invalid port given");

    HttpServer::new(|| {
        App::new()
            .service(healthz)
            .service(ping)
            .service(showTables)
    })
    .bind((address, port))?
    .run()
    .await
}
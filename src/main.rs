use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use db::db_get_handle;
use routes::grid_chunk;
use sqlx::{Pool, Postgres};

mod db;
mod routes;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "0.0.0.0";
    let port = 8080;

    let connection = db_get_handle().await;
    println!("[server] db connection established");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let data = Data::new(AppState {
            db: connection.clone(),
        });

        println!("[server] running on {}:{}", ip, port);

        App::new()
            .wrap(cors)
            .app_data(data)
            .service(grid_chunk)
    })
    .bind((ip, port))?
    .run()
    .await
}

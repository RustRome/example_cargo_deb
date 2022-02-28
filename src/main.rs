use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    static_path: String,
    listen_port: u16,
    listen_host: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = envy::from_env::<Config>().unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", &config.static_path).index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((config.listen_host, config.listen_port))?
    .run()
    .await
}

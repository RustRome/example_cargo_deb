use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

struct Config {
    static_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = Config {
        static_path: "/var/www/example-project/".to_owned(),
    };
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", &config.static_path).index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

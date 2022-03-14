use actix_files::Files;
use actix_web::{
    http::StatusCode,
    middleware::Logger,
    web::{self, Data, Json},
    App, HttpResponse, HttpResponseBuilder, HttpServer,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use model::Conn;
use serde_derive::Deserialize;
mod model;
mod schema;
#[macro_use]
extern crate diesel;

#[derive(Deserialize)]
struct Config {
    static_path: String,
    listen_port: u16,
    listen_host: String,
    database_host: String,
    database_name: String,
    database_user: String,
    database_password: String,
}

#[derive(Deserialize)]
struct EventCreate {
    title: String,
}

async fn add_event(event: Json<EventCreate>, db_state: Data<DbState>) -> String {
    if let Err(e) = model::Event::insert(&db_state.conn(), &event.title) {
        format!("Err {:?}", e)
    } else {
        "Ok".to_owned()
    }
}

async fn list_event(db_state: Data<DbState>) -> HttpResponse {
    match model::Event::list(&db_state.conn()) {
        Ok(ok) => HttpResponseBuilder::new(StatusCode::OK).json(ok),
        Err(e) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Err {:?}", e).as_bytes().to_owned()),
    }
}

async fn add_food(event: Json<EventCreate>, db_state: Data<DbState>) -> String {
    if let Err(e) = model::Food::insert(&db_state.conn(), &event.title) {
        format!("Err {:?}", e)
    } else {
        "Ok".to_owned()
    }
}

async fn list_food(db_state: Data<DbState>) -> HttpResponse {
    match model::Food::list(&db_state.conn()) {
        Ok(ok) => HttpResponseBuilder::new(StatusCode::OK).json(ok),
        Err(e) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Err {:?}", e).as_bytes().to_owned()),
    }
}

struct DbState {
    pool: Pool<ConnectionManager<PgConnection>>,
}
impl DbState {
    fn new(conn_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(conn_url);
        let pool = Pool::builder().build(manager).unwrap();
        DbState { pool }
    }
    fn conn(&self) -> Conn {
        self.pool.get().unwrap()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = envy::from_env::<Config>().unwrap();

    log::info!("starting HTTP server at http://localhost:8080");
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config.database_user, config.database_password, config.database_host, config.database_name
    );

    let data = Data::new(DbState::new(&database_url));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::resource("list_events").route(web::get().to(list_event)))
            .service(web::resource("add_event").route(web::post().to(add_event)))
            .service(web::resource("list_foods").route(web::get().to(list_food)))
            .service(web::resource("add_food").route(web::post().to(add_food)))
            .service(Files::new("/", &config.static_path).index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((config.listen_host, config.listen_port))?
    .run()
    .await
}

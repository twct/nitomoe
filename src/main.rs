use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use config::database::establish_db_connection;
use dotenv::dotenv;

mod config;
mod env;
mod macros;
mod models;
mod repos;
mod schema;

async fn default_service() -> impl Responder {
    "Hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_connection = web::Data::new(establish_db_connection(&env::DATABASE_URL));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_connection.clone())
            .default_service(web::to(default_service))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

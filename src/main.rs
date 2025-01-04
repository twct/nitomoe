use actix_files as fs;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use config::database::establish_db_connection;
use dotenv::dotenv;
use views::admin_view::AdminDashboard;

mod config;
mod env;
mod macros;
mod models;
mod repos;
mod schema;
mod views;

async fn default_service() -> impl Responder {
    "Hello"
}

#[get("/admin")]
async fn admin_dashboard() -> impl Responder {
    let context = AdminDashboard {};
    let rendered = context.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
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
            .service(admin_dashboard)
            .service(fs::Files::new("/css", "./static/css").show_files_listing())
            .default_service(web::to(default_service))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

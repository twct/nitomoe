use actix_files as fs;
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use args::{process_args, Args};
use clap::Parser;
use config::database::establish_db_connection;
use dotenv::dotenv;
use views::{
    admin_view::{AdminAuth, AdminDashboard},
    render_view,
};

mod args;
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
    render_view(AdminDashboard {})
}

#[get("/admin/auth")]
async fn admin_login() -> impl Responder {
    render_view(AdminAuth {})
}

async fn run_server() -> std::io::Result<()> {
    let db_connection = web::Data::new(establish_db_connection(&env::DATABASE_URL));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_connection.clone())
            .service(admin_dashboard)
            .service(admin_login)
            .service(
                fs::Files::new("/css", "./static/css")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(fs::Files::new("/img", "./assets/img"))
            .default_service(web::to(default_service))
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if std::env::args().len() > 1 {
        let args = Args::parse();
        process_args(args).await
    } else {
        run_server().await
    }
}

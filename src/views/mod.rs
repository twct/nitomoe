use actix_web::{HttpResponse, Responder};
use askama::Template;

pub mod admin_view;

pub fn render_view<T: Template>(context: T) -> impl Responder {
    match context.render() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            log::error!("Error rendering template: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

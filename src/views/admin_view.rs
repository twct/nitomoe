use askama::Template;

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct AdminDashboard {}

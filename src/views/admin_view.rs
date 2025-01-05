use askama::Template;

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct AdminDashboard {}

#[derive(Template)]
#[template(path = "admin/auth.html")]
pub struct AdminAuth {}

use std::env;

use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<&'static str> = Lazy::new(|| {
    static VALUE: Lazy<String> = Lazy::new(|| {
        env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/nitomoe".to_string())
    });
    &VALUE
});
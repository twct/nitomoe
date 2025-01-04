#[macro_export]
macro_rules! define_env {
    ($key:ident, $default:expr) => {
        pub static $key: once_cell::sync::Lazy<&'static str> = once_cell::sync::Lazy::new(|| {
            static VALUE: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
                std::env::var(stringify!($key)).unwrap_or_else(|_| $default.to_string())
            });
            &VALUE
        });
    };
}

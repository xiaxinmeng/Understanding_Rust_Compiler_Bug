rust
pub const VERSION: &'static str = match option_env!("VERSION") {
    Some(v) => v,
    None => "dev",
};

rust
pub const VERSION: &str= match option_env!("VERSION") {
    Some(v) => v,
    None => "dev",
};

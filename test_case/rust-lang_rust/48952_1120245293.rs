rust
pub const VERSION: &'static str = option_env!("VERSION").unwrap_or_else(|| "dev");

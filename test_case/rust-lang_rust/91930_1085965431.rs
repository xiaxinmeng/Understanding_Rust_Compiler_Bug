rust
pub const FOO: usize = option_env!("FOO")
    .unwrap_or("123456")
    .parse()
    .unwrap();

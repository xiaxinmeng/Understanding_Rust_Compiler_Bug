rust
env::var("SQLX_OFFLINE")
    .map(|s| s.eq_ignore_ascii_case("true") || s == "1")
    .unwrap_or(false)

rs
error: implementation of `sqlx::Decode` is not general enough
   --> src/models.rs:183:5
    |
183 |     Decode,
    |     ^^^^^^ implementation of `sqlx::Decode` is not general enough
    |
    = note: `std::string::String` must implement `sqlx::Decode<'0, sqlx::Postgres>`, for some specific lifetime `'0`...
    = note: ...but it actually implements `sqlx::Decode<'r, sqlx::Postgres>`
    = note: this error originates in the derive macro `Decode` (in Nightly builds, run with -Z macro-backtrace for more info)

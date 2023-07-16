
Jan 09 21:58:29.356 INFO kablam! error[E0283]: type annotations required: cannot resolve `std::string::String: std::cmp::PartialEq<_>`
Jan 09 21:58:29.356 INFO kablam!   --> src/migrations/migration.rs:60:15
Jan 09 21:58:29.356 INFO kablam!    |
Jan 09 21:58:29.356 INFO kablam! 60 |         files.contains(&"down.sql".into()) && files.contains(&"up.sql".into())
Jan 09 21:58:29.357 INFO kablam!    |               ^^^^^^^^
Jan 09 21:58:29.357 INFO kablam! 

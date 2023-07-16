
error: cannot find attribute `serde` in this scope
  --> src/error.rs:11:3
   |
11 | #[serde(remote = "StatusCode")]
   |   ^^^^^

error: cannot find attribute `serde` in this scope
  --> src/error.rs:12:24
   |
12 | struct StatusCodeDef(#[serde(getter = "StatusCode::as_u16")] u16);
   |                        ^^^^^

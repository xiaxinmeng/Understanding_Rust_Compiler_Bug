
error: invalid path URI: expected EOF but found non-ASCII byte 226 at index 1
 --> src/main.rs:1:17
  |
1 | #[rocket::get("/♥")]
  |                 ^^
  |
  = help: expected path in origin form: "/path/<param>"

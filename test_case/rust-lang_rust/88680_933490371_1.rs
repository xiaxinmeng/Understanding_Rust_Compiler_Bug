
error: malformed `path` attribute input
 --> ...
  |
2 | #[path = concat!(env!("OUT_DIR"), "/v1_22/mod.rs")]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[path = "file"]`

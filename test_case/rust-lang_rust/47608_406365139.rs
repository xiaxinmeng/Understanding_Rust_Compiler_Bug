
error[E...]: The attribute `serde` is provided by derive(Serialize) or derive(Deserialize); one of these must be derived on `CellIndex` for this attribute to be available
 --> src/main.rs:4:1
  |
4 | #[serde(untagged)]
  | ^^^^^^^^^^^^^^^^^^

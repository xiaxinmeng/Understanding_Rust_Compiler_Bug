console
error[E0277]: the trait bound `S: _IMPL_DESERIALIZE_FOR_S::_serde::Serialize` is not satisfied
 --> src/main.rs:7:5
  |
7 |     serde_json::to_string(&S);
  |     ^^^^^^^^^^^^^^^^^^^^^ the trait `_IMPL_DESERIALIZE_FOR_S::_serde::Serialize` is not implemented for `S`
  |
  = note: required by `serde_json::ser::to_string`


error[E0277]: the trait bound `Bar: _IMPL_DESERIALIZE_FOR_Root::_serde::Deserialize<'_>` is not satisfied
 --> src/main.rs:7:5
  |
7 |     /// doc comment included as the snippet
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `_IMPL_DESERIALIZE_FOR_Root::_serde::Deserialize<'_>` is not implemented for `Bar`
  |
  = note: required by `_IMPL_DESERIALIZE_FOR_Root::_serde::de::SeqAccess::next_element`

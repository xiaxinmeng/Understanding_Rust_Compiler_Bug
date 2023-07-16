
error[E0277]: the trait bound `toml::de::Error: _IMPL_DESERIALIZE_FOR_Options::_serde::Deserialize<'_>` is not satisfied
  --> src/lib.rs:9:5
   |
9  |     toml::from_str("")?
   |     ^^^^^^^^^^^^^^ the trait `_IMPL_DESERIALIZE_FOR_Options::_serde::Deserialize<'_>` is not implemented for `toml::de::Error`
   | 
  ::: /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/toml-0.5.6/src/de.rs:77:8
   |
77 |     T: de::Deserialize<'de>,
   |        -------------------- required by this bound in `toml::de::from_str`
   |
   = note: required because of the requirements on the impl of `_IMPL_DESERIALIZE_FOR_Options::_serde::Deserialize<'_>` for `std::result::Result<Options, toml::de::Error>`

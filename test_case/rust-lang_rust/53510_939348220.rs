
error[E0277]: the trait bound `toml::de::Error: Deserialize<'_>` is not satisfied
  --> src/lib.rs:9:5
   |
9  |     toml::from_str("")?
   |     ^^^^^^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `toml::de::Error`
   |
   = note: required because of the requirements on the impl of `Deserialize<'_>` for `Result<Options, toml::de::Error>`
note: required by a bound in `toml::from_str`
  --> /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/toml-0.5.8/src/de.rs:77:8
   |
77 |     T: de::Deserialize<'de>,
   |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `toml::from_str`

For more information about this error, try `rustc --explain E0277`.

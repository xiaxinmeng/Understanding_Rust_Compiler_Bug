
error: anonymous parameters are deprecated and will be removed in the next edition.
 --> src/lib.rs:4:13
  |
4 |     fn func(u8);
  |             ^^ help: Try naming the parameter or explicitly ignoring it: `_: u8`
  |
[...]
  = note: #[deny(anonymous_parameters)] implied by #[deny(rust_2018_compatibility)]


error[[E0562]](https://doc.rust-lang.org/nightly/error_codes/E0562.html): `impl Trait` only allowed in function and inherent method return types, not in closure return types
 --> src/main.rs:5:36
  |
5 |     let f = for<'a> |a: &'a u8| -> impl 'a + Fn() -> &'a u8 {
  |                                    ^^^^^^^^^^^^^^^^^^^^^^^^

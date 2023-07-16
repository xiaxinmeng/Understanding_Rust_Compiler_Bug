
Error[[E0072]](https://doc.rust-lang.org/nightly/error_codes/E0072.html): recursive types `Foo` and `Bar` have infinite size
 --> src/main.rs:1:1
  |
1 | struct Foo<'a>(u64, Bar<'a>);
  | ^^^^^^^^^^^^^^      ------- recursive without indirection
2 | struct Bar<'a>(u64, Foo<'a>);
  | ^^^^^^^^^^^^^^      ------- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
1 ~ struct Foo<'a>(u64, Box<Bar<'a>>);
2 ~ struct Bar<'a>(u64, Box<Foo<'a>>);
  |

error[[E0392]](https://doc.rust-lang.org/nightly/error_codes/E0392.html): parameter `'a` is never used
 --> src/main.rs:1:12
  |
1 | struct Foo<'a>(u64, Bar<'a>);
  |            ^^ unused parameter
  |
  = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[[E0392]](https://doc.rust-lang.org/nightly/error_codes/E0392.html): parameter `'a` is never used
 --> src/main.rs:2:12
  |
2 | struct Bar<'a>(u64, Foo<'a>);
  |            ^^ unused parameter
  |
  = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
 

error[E0390]: only a single inherent implementation marked with `#[lang = "slice"]` is allowed for the `[T]` primitive
   --> src/liballoc/slice.rs:169:1
    |
169 | / impl [u8] {
170 | |     fn foobar(&self) {}
171 | | }
    | |_^
    |
help: consider using a trait to implement these methods
   --> src/liballoc/slice.rs:169:1
    |
169 | / impl [u8] {
170 | |     fn foobar(&self) {}
171 | | }
    | |_^

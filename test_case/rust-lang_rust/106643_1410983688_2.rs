console
error[E0046]: not all trait items implemented, missing one of: `read`, `read_buf`
   --> src/main.rs:3:1
    |
3   | impl std::io::Read for MyRead {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing one of `read`, `read_buf` in implementation
    |
note: required because of this annotation
   --> /git/rust/library/std/src/io/mod.rs:552:1
    |
552 | #[rustc_must_implement_one_of(read, read_buf)]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

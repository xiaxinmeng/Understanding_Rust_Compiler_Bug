
$ cargo +nightly build
   Compiling foo v0.1.0 (/home/alex/code/foo)
attr:
input: extern "C" {
    fn foo(x: i32, ..., ...);
}
warning: foreign function is never used: `foo`
 --> main.rs:3:5
  |
3 |     fn foo(x: i32, ...);
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.21s

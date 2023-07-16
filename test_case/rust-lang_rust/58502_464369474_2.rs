
$ cargo build
   Compiling mac v0.1.0 (/home/euclio/scratch/hello/mac)
   Compiling hello v0.1.0 (/home/euclio/scratch/hello)
warning: struct is never constructed: `Foo`
 --> src/main.rs:4:10
  |
4 | wrapper!(struct Foo {});
  |          ^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.43s

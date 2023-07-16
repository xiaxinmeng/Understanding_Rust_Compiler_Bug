
$ mv src/foo.rs src/foo/mod.rs
$ echo "include!(\"foo/mod.rs\");" >src/baz.rs
$ cargo check
    Checking include-bug v0.1.0 (/path/to/include-bug)
warning: constant is never used: `WORLD`
 --> src/foo/bar.rs:1:1
  |
1 | const WORLD: usize = 42;
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: constant is never used: `WORLD`
 --> src/foo/bar.rs:1:1
  |
1 | const WORLD: usize = 42;
  | ^^^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.08s

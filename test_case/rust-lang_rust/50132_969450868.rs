
$ cargo init --lib include-bug
$ cd include-bug
$ cat Cargo.toml
[...]
edition = "2018"
[...]
$ mkdir -p src/foo
$ echo "mod foo; mod baz;" >src/lib.rs
$ echo "mod bar;" >src/foo.rs
$ echo "const WORLD: usize = 42;" >src/foo/bar.rs
$ echo "include!(\"foo.rs\");" >src/baz.rs
$ cargo check
    Checking include-bug v0.1.0 (/path/to/include-bug)
error[E0583]: file not found for module `bar`
 --> src/foo.rs:1:1
  |
1 | mod bar;
  | ^^^^^^^^
  |
  = help: to create the module `bar`, create file "src/bar.rs" or "src/bar/mod.rs"

error: aborting due to previous error

For more information about this error, try `rustc --explain E0583`.
error: could not compile `include-bug`

To learn more, run the command again with --verbose.

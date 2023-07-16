rust
$ cat src/lib.rs
#![warn(intra_doc_resolution_failure)]

extern crate a;

/// Link to [`NotHere`]
pub use a::Bar;
$ cat ../a/src/lib.rs 
// crate a:
pub struct Foo;

/// Link to [Food]
pub struct Bar;
$ cargo +stage1 doc
 Documenting a v0.1.0 (/home/joshua/src/test-rustdoc/a)
    Checking a v0.1.0 (/home/joshua/src/test-rustdoc/a)
warning: `[Food]` cannot be resolved, ignoring it.
 --> /home/joshua/src/test-rustdoc/a/src/lib.rs:4:14
  |
4 | /// Link to [Food]
  |              ^^^^ cannot be resolved, ignoring
  |
  = note: `#[warn(intra_doc_link_resolution_failure)]` on by default
  = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`

warning: 1 warning emitted

 Documenting b v0.1.0 (/home/joshua/src/test-rustdoc/b)
    Finished dev [unoptimized + debuginfo] target(s) in 1.43s

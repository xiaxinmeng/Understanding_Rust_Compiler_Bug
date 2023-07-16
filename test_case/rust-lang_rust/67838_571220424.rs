
❯ cargo expand | grep doc
    Checking spandoc-example v0.1.0 (/home/jlusby/git/rust/spandoc-example)
warning: unused doc comment
 --> src/lib.rs:3:5
  |
3 |     /// Span in example
  |     ^^^^^^^^^^^^^^^^^^^
4 |     42
  |     -- rustdoc does not generate documentation for expressions
  |
  = note: `#[warn(unused_doc_comments)]` on by default
    Finished check [unoptimized + debuginfo] target(s) in 0.07s

    # [ doc = " Span in example" ] 42

rust
error: this constant cannot be used
  --> src/main.rs:16:1
   |
16 | const X: u64 = *wat(42);
   | ^^^^^^^^^^^^^^^--------^
   |                |
   |                dangling pointer was dereferenced

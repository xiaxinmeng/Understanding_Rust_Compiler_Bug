rust
error[E0404]: expected trait, found keyword `fn`
 --> src/lib.rs:1:11
  |
4 | fn map(f: impl fn()) {}
  |                ^^ not a trait

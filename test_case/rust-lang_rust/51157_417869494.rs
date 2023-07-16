Rust
error[E0596]: cannot borrow `*f` as mutable, as `f` is not declared as mutable
 --> src/main.rs:4:9
  |
2 | fn test(f: Box<FnMut()>) {
  |         - help: consider changing this to be mutable: `mut f`
3 |     let _f = move || {
4 |         f()
  |         ^ cannot borrow as mutable

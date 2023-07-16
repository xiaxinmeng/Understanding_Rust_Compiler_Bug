rust
pub async fn f() {
    content::should::not::matter();
}
error[E0433]: failed to resolve: could not resolve path `content::should::not::matter`
 --> body_error.rs:2:5
  |
2 |     content::should::not::matter();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not resolve path `content::should::not::matter`
  |
  = note: this error was originally ignored because you are running `rustdoc`
  = note: try running again with `rustc` or `cargo check` and you may get a more detailed error

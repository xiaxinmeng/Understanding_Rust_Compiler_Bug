
$ rustdoc +beta --version
rustdoc 1.46.0-beta.2 (6f959902b 2020-07-23)
$ rustdoc +beta async_error.rs  --edition 2018
# no output
$ rustdoc +nightly async_error.rs  --edition 2018
error[E0433]: failed to resolve: could not resolve path `content::should::not::matter`
 --> async_error.rs:2:5
  |
2 |     content::should::not::matter();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not resolve path `content::should::not::matter`
  |
  = note: this error was originally ignored because you are running `rustdoc`
  = note: try running again with `rustc` or `cargo check` and you may get a more detailed error

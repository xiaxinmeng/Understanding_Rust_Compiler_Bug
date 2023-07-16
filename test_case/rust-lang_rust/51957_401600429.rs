
warning: float has excessive precision
 --> src/main.rs:2:34
  |
2 |     println!("Hello, world! {}", 4444444444_f32 );
  |                                  ^^^^^^^^^^^^^^ help: consider changing the type or truncating it to: `4_444_444_700`
  |
  = note: #[warn(excessive_precision)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.211/index.html#excessive_precision

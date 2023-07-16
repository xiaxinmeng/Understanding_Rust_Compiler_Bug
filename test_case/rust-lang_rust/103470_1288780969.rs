
warning: hidden lifetime parameters in types are deprecated
 --> src/main.rs:7:12
  |
7 | fn run(cx: Context) {}
  |            ^^^^^^^ expected lifetime parameter
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
1 | #![warn(rust_2018_idioms)]
  |         ^^^^^^^^^^^^^^^^
  = note: `#[warn(elided_lifetimes_in_paths)]` implied by `#[warn(rust_2018_idioms)]`
help: indicate the anonymous lifetime
  |
7 | fn run(cx: Context<'_>) {}
  |                   ++++

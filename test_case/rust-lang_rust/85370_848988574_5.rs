
> RUST_BACKTRACE=1 cargo +nightly check --tests
error: ~/ice/target/debug/deps/libice-3fb954df89f48978.so: undefined symbol: _ZN2v86Object19GetOwnPropertyNamesENS_5LocalINS_7ContextEEE
 --> tests/integration.rs:1:5
  |
1 | use ice::proc;
  |     ^^^

error: aborting due to previous error

error: could not compile `ice`

To learn more, run the command again with --verbose.

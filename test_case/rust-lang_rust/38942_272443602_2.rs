
% rustup default nightly-2017-01-08
% rustup target add i686-apple-darwin
% cargo build --target i686-apple-darwin
   [...]
   Compiling cocoa v0.3.3
error: internal compiler error: /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc_trans/mir/constant.rs:682: Unexpected non-fat-pointer operand

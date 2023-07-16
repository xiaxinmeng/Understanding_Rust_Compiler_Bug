
(-bash@build-server) ~/.../src/test ▶️ rustdoc +stage1 rustdoc/macro-in-closure.rs
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/joshua/src/rust/src/librustc_hir/definitions.rs:358:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
(-bash@build-server) ~/.../src/test ▶️ cat !$
cat rustdoc/macro-in-closure.rs
// Regression issue for rustdoc ICE encountered in PR #65252.

#![feature(decl_macro)]

fn main() {
    || {
        macro m() {}
    };
}

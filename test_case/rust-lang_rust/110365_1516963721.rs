
  ~/devspace/.other/chunk-list  stable $ cargo +stage2 miri test
Preparing a sysroot for Miri (target: x86_64-unknown-linux-gnu)... done
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: CargoMetadata { stderr: "error: the `-Z` flag is only accepted on the nightly channel of Cargo, but this is the `stable` channel\nSee https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.\n" }', src/tools/miri/cargo-miri/src/util.rs:224:80
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

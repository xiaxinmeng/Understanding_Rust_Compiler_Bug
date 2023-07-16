
error: failed to run custom build command for `crossbeam-utils v0.7.2`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/crossbeam-utils-a40e83968eeae062/build-script-build` (exit code: 101)
  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { kind: Other("missing patch version") }', /cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-1.0.0/src/lib.rs:128:20

plain

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/ahash-e3b4eb2c0ecc2acf/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  --- stderr
  --- stderr
  thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', library/core/src/str_bytes.rs:1524:69
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:34

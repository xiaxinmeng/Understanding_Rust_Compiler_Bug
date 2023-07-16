
$ RUSTFLAGS="-C target-cpu=native -Zbuild-std" cargo +nightly build --release && cp target/release/memchr-perf-regression ./regress-nightly_2021-03-10-native-buildstd
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -C target-cpu=native -Zbuild-std --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 1)
  --- stderr
  error: unknown debugging option: `build-std`

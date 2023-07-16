plain
* highest error code: E0786
Found 502 error codes
Found 0 error codes with no tests
Done!
thread '<unnamed>' panicked at 'cmd.exec() failed with Error during execution of `cargo metadata`:     Updating git repository `https://github.com/bjorn3/rust-ar.git`
warning: spurious network error (2 tries remaining): request failed with status code: 504; class=Http (34)
warning: spurious network error (1 tries remaining): request failed with status code: 504; class=Http (34)
error: failed to get `ar` as a dependency of package `rustc_codegen_cranelift v0.1.0 (D:\a\rust\rust\compiler\rustc_codegen_cranelift)`
Caused by:
  failed to load source for dependency `ar`

Caused by:
Caused by:
  Unable to update https://github.com/bjorn3/rust-ar.git?branch=do_not_remove_cg_clif_ranlib#de9ab0e5

Caused by:
  failed to clone into: C:\Users\runneradmin\.cargo\git\db\rust-ar-9b35aff8ad678e06
Caused by:
  network failure seems to have happened
  network failure seems to have happened
  if a proxy or similar is necessary `net.git-fetch-with-cli` may help here
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli
Caused by:
Caused by:
  request failed with status code: 504; class=Http (34)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src\tools\tidy\src/main.rs:77:9


command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\rust-tidy.exe" "D:\\a\\rust\\rust" "\\\\?\\D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0\\bin\\cargo.exe" "D:\\a\\rust\\rust\\build" "8"


Build completed unsuccessfully in 0:03:55
Build completed unsuccessfully in 0:03:55
make: *** [Makefile:72: ci-subset-1] Error 1

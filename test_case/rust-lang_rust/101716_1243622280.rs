plain
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
thread '<unnamed>' panicked at 'cmd.exec() failed with `cargo metadata` exited with an error:     Updating git repository `https://github.com/bjorn3/rust-ar.git`
warning: spurious network error (2 tries remaining): request failed with status code: 503; class=Http (34)
warning: spurious network error (1 tries remaining): request failed with status code: 503; class=Http (34)
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
  if a proxy or similar is necessary `net.git-fetch-with-cli` may help here
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli

Caused by:
  request failed with status code: 503; class=Http (34)', src\tools\tidy\src\deps.rs:341:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src\tools\tidy\src/main.rs:77:9
Build completed unsuccessfully in 0:01:06
make: *** [Makefile:83: ci-mingw-subset-1] Error 1

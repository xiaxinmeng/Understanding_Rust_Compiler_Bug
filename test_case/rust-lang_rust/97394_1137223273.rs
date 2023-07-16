plain
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
thread '<unnamed>' panicked at 'cmd.exec() failed with `cargo metadata` exited with an error:     Updating git repository `https://github.com/bjorn3/rust-ar.git`
warning: spurious network error (2 tries remaining): failed to connect to github.com: Connection timed out; class=Os (2)
warning: spurious network error (1 tries remaining): failed to connect to github.com: Connection timed out; class=Os (2)
error: failed to get `ar` as a dependency of package `rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)`
Caused by:
  failed to load source for dependency `ar`

Caused by:
Caused by:
  Unable to update https://github.com/bjorn3/rust-ar.git?branch=do_not_remove_cg_clif_ranlib#de9ab0e5

Caused by:
  failed to clone into: /cargo/git/db/rust-ar-9b35aff8ad678e06
Caused by:
Caused by:
  failed to connect to github.com: Connection timed out; class=Os (2)', src/tools/tidy/src/deps.rs:320:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src/tools/tidy/src/main.rs:92:9

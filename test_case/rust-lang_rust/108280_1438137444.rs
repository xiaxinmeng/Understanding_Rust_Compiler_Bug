
thread 'main' panicked at 'self.symlink_file(&rustfmt_path, &legacy_rustfmt) failed with File exists (os error 17)', download.rs:346:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9aa5c24b7d763fb98d998819571128ff2eb8a3ca/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/9aa5c24b7d763fb98d998819571128ff2eb8a3ca/library/core/src/panicking.rs:64:14
   2: bootstrap::download::<impl bootstrap::config::Config>::maybe_download_rustfmt
             at ./src/bootstrap/download.rs:346:17
   3: bootstrap::config::Config::initial_rustfmt
             at ./src/bootstrap/config.rs:1562:28
   4: bootstrap::Build::initial_rustfmt
             at ./src/bootstrap/lib.rs:313:17
   5: bootstrap::Build::build
             at ./src/bootstrap/lib.rs:658:18
   6: bootstrap::main
             at ./src/bootstrap/bin/main.rs:55:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/9aa5c24b7d763fb98d998819571128ff2eb8a3ca/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

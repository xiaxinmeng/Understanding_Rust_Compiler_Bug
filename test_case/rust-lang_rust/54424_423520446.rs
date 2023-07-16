plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0e2c65ae
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:54:54]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:54:59]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:56:35]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:56:46]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:58:18] error[E0492]: cannot borrow a constant which may contain interior mutability or non-`Sync` data. If your data is `Sync`, create a static instead
[00:58:18]    --> librustc/middle/resolve_lifetime.rs:349:39
[00:58:18]     |
[00:58:18] 349 | const ROOT_SCOPE: ScopeRef<'static> = &Scope::Root;
[00:58:18] 
[00:58:30] error: aborting due to previous error
[00:58:30] 
[00:58:30] For more information about this error, try `rustc --explain E0492`.
[00:58:30] For more information about this error, try `rustc --explain E0492`.
[00:58:30] error: Could not compile `rustc`.
[00:58:30] 
[00:58:30] To learn more, run the command again with --verbose.
[00:58:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:58:30] expected success, got: exit code: 101
[00:58:30] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:58:30] travis_fold:end:stage1-rustc

[00:58:30] travis_time:end:stage1-rustc:start=1537534055714089615,finish=1537534330162767347,duration=274448677732


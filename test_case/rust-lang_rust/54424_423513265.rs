plain
[00:25:49]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:25:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:27:27]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:27:38]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:29:17] error[E0492]: cannot borrow a constant which may contain interior mutability or non-`Sync` data. If your data is `Sync`, create a static instead
[00:29:17]    --> librustc/middle/resolve_lifetime.rs:349:39
[00:29:17]     |
[00:29:17] 349 | const ROOT_SCOPE: ScopeRef<'static> = &Scope::Root;
[00:29:17] 
[00:29:31] error: aborting due to previous error
[00:29:31] 
[00:29:31] For more information about this error, try `rustc --explain E0492`.
[00:29:31] For more information about this error, try `rustc --explain E0492`.
[00:29:31] error: Could not compile `rustc`.
[00:29:31] 
[00:29:31] To learn more, run the command again with --verbose.
[00:29:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:29:31] expected success, got: exit code: 101
[00:29:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:29:31] travis_fold:end:stage1-rustc

[00:29:31] travis_time:end:stage1-rustc:start=1537532308057024168,finish=1537532590662682214,duration=282605658046


[00:29:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:29:31] Build completed unsuccessfully in 0:24:24
[00:29:31] make: *** [all] Error 1
[00:29:31] Makefile:28: recipe for target 'all' failed
91404 ./obj/build/x86_64-unknown-linux-gnu/stage1
91380 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
77952 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
74188 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot

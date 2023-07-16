plain
[00:16:24]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:16:33]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:16:34]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:16:35]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:35] error: expected one of `.`, `;`, `?`, or an operator, found `if`
[00:16:35]     |
[00:16:35] 347 |                 }
[00:16:35] 347 |                 }
[00:16:35]     |                  - expected one of `.`, `;`, `?`, or an operator here
[00:16:35] ...
[00:16:35] 352 |                 if e.span.ctxt().outer().expn_info()
[00:16:35]     |                 ^^ unexpected token
[00:16:36] error[E0277]: no implementation for `bool | rustc::ty::TyKind<'_>`
[00:16:36]   --> librustc_lint/unused.rs:63:50
[00:16:36]    |
[00:16:36]    |
[00:16:36] 63 |             ty::Tuple(ref tys) if tys.is_empty() |
[00:16:36]    |                                                  ^ no implementation for `bool | rustc::ty::TyKind<'_>`
[00:16:36]    |
[00:16:36]    = help: the trait `std::ops::BitOr<rustc::ty::TyKind<'_>>` is not implemented for `bool`
[00:16:37] error: aborting due to 2 previous errors
[00:16:37] 
[00:16:37] For more information about this error, try `rustc --explain E0277`.
[00:16:37] error: Could not compile `rustc_lint`.
[00:16:37] error: Could not compile `rustc_lint`.
[00:16:37] warning: build failed, waiting for other jobs to finish...
[00:16:46] error: build failed
[00:16:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:46] expected success, got: exit code: 101
[00:16:46] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:16:46] travis_fold:end:stage0-rustc

[00:16:46] travis_time:end:stage0-rustc:start=1538478357805551875,finish=1538479049469217216,duration=691663665341


[00:16:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:46] Build completed unsuccessfully in 0:12:25
[00:16:46] make: *** [all] Error 1
[00:16:46] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:025cf29b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

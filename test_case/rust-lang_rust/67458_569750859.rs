plain
2019-12-30T17:39:10.8589086Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T17:39:11.8403410Z ##[command]git config gc.auto 0
2019-12-30T17:39:11.8407674Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T17:39:11.8411778Z ##[command]git config --get-all http.proxy
2019-12-30T17:39:11.8414442Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67458/merge:refs/remotes/pull/67458/merge
---
2019-12-30T17:48:21.9889471Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-30T17:48:22.1818790Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-12-30T17:48:22.6630523Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-30T17:48:24.0581937Z error[E0308]: mismatched types
2019-12-30T17:48:24.0582330Z     --> src/librustc_codegen_ssa/back/write.rs:1566:62
2019-12-30T17:48:24.0582603Z      |
2019-12-30T17:48:24.0582945Z 1566 |                     None => Message::Done::<B> { result: Err(()), worker_id },
2019-12-30T17:48:24.0583570Z      |                                                              ^^ expected enum `std::option::Option`, found `()`
2019-12-30T17:48:24.0583793Z      |
2019-12-30T17:48:24.0584084Z      = note:   expected enum `std::option::Option<back::write::WorkerFatalError>`
2019-12-30T17:48:24.0584374Z 
2019-12-30T17:48:25.1687642Z error: aborting due to previous error
2019-12-30T17:48:25.1687808Z 
2019-12-30T17:48:25.1696282Z For more information about this error, try `rustc --explain E0308`.
2019-12-30T17:48:25.1696282Z For more information about this error, try `rustc --explain E0308`.
2019-12-30T17:48:25.1775373Z error: could not compile `rustc_codegen_ssa`.
2019-12-30T17:48:25.1790661Z warning: build failed, waiting for other jobs to finish...
2019-12-30T17:48:29.5072170Z error: build failed
2019-12-30T17:48:29.5098063Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-30T17:48:29.5118910Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-30T17:48:29.5119567Z Build completed unsuccessfully in 0:05:49
2019-12-30T17:48:29.5182336Z == clock drift check ==
2019-12-30T17:48:29.5194380Z   local time: Mon Dec 30 17:48:29 UTC 2019
2019-12-30T17:48:29.5194380Z   local time: Mon Dec 30 17:48:29 UTC 2019
2019-12-30T17:48:29.8068786Z   network time: Mon, 30 Dec 2019 17:48:29 GMT
2019-12-30T17:48:29.8078195Z == end clock drift check ==
2019-12-30T17:48:30.8787592Z 
2019-12-30T17:48:30.8894981Z ##[error]Bash exited with code '1'.
2019-12-30T17:48:30.8928475Z ##[section]Starting: Checkout
2019-12-30T17:48:30.8930587Z ==============================================================================
2019-12-30T17:48:30.8930662Z Task         : Get sources
2019-12-30T17:48:30.8930713Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

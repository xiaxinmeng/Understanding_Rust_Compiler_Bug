plain
2019-12-23T18:17:25.5989238Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T18:17:25.6215213Z ##[command]git config gc.auto 0
2019-12-23T18:17:26.6255204Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T18:17:26.6258304Z ##[command]git config --get-all http.proxy
2019-12-23T18:17:26.6261231Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67458/merge:refs/remotes/pull/67458/merge
---
2019-12-23T18:26:15.6413806Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2019-12-23T18:26:15.8289819Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-12-23T18:26:16.3169875Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-12-23T18:26:17.6531425Z error[E0308]: mismatched types
2019-12-23T18:26:17.6533220Z     --> src/librustc_codegen_ssa/back/write.rs:1566:62
2019-12-23T18:26:17.6534019Z      |
2019-12-23T18:26:17.6534486Z 1566 |                     None => Message::Done::<B> { result: Err(()), worker_id },
2019-12-23T18:26:17.6534988Z      |                                                              ^^ expected enum `std::option::Option`, found `()`
2019-12-23T18:26:17.6535406Z      |
2019-12-23T18:26:17.6535844Z      = note:   expected enum `std::option::Option<back::write::WorkerFatalError>`
2019-12-23T18:26:17.6536429Z 
2019-12-23T18:26:18.7797464Z error: aborting due to previous error
2019-12-23T18:26:18.7797735Z 
2019-12-23T18:26:18.7798365Z For more information about this error, try `rustc --explain E0308`.
2019-12-23T18:26:18.7798365Z For more information about this error, try `rustc --explain E0308`.
2019-12-23T18:26:18.7798636Z error: could not compile `rustc_codegen_ssa`.
2019-12-23T18:26:18.7798902Z warning: build failed, waiting for other jobs to finish...
2019-12-23T18:26:20.5943322Z error: build failed
2019-12-23T18:26:20.5966999Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T18:26:20.5979596Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T18:26:20.5979853Z Build completed unsuccessfully in 0:05:40
2019-12-23T18:26:20.6033147Z == clock drift check ==
2019-12-23T18:26:20.6049524Z   local time: Mon Dec 23 18:26:20 UTC 2019
2019-12-23T18:26:20.6049524Z   local time: Mon Dec 23 18:26:20 UTC 2019
2019-12-23T18:26:21.7810364Z   network time: Mon, 23 Dec 2019 18:26:20 GMT
2019-12-23T18:26:21.7810891Z == end clock drift check ==
2019-12-23T18:26:21.9723612Z 
2019-12-23T18:26:21.9824996Z ##[error]Bash exited with code '1'.
2019-12-23T18:26:21.9859653Z ##[section]Starting: Checkout
2019-12-23T18:26:21.9861485Z ==============================================================================
2019-12-23T18:26:21.9861534Z Task         : Get sources
2019-12-23T18:26:21.9861593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

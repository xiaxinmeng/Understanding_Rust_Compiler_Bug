plain
2020-01-15T19:08:26.6760851Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T19:08:26.6773313Z ##[command]git config gc.auto 0
2020-01-15T19:08:26.6780237Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T19:08:26.6784751Z ##[command]git config --get-all http.proxy
2020-01-15T19:08:26.6791593Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67954/merge:refs/remotes/pull/67954/merge
---
2020-01-15T19:16:39.2133341Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-01-15T19:16:39.3983728Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-01-15T19:16:39.8710710Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2020-01-15T19:16:55.0219033Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-01-15T19:17:17.3845464Z error[E0599]: no method named `generic_pass` found for type `rustc_data_structures::profiling::SelfProfilerRef` in the current scope
2020-01-15T19:17:17.3846801Z    --> src/librustc_codegen_llvm/back/lto.rs:596:23
2020-01-15T19:17:17.3852623Z     |
2020-01-15T19:17:17.3853759Z 596 |             cgcx.prof.generic_pass("LTO passes").run(|| {
2020-01-15T19:17:17.3854467Z     |                       ^^^^^^^^^^^^ method not found in `rustc_data_structures::profiling::SelfProfilerRef`
2020-01-15T19:17:18.9005670Z error: aborting due to previous error
2020-01-15T19:17:18.9006572Z 
2020-01-15T19:17:18.9007504Z For more information about this error, try `rustc --explain E0599`.
2020-01-15T19:17:18.9171825Z error: could not compile `rustc_codegen_llvm`.
2020-01-15T19:17:18.9171825Z error: could not compile `rustc_codegen_llvm`.
2020-01-15T19:17:18.9172326Z 
2020-01-15T19:17:18.9172851Z To learn more, run the command again with --verbose.
2020-01-15T19:17:18.9229325Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-15T19:17:18.9240516Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-15T19:17:18.9240786Z Build completed unsuccessfully in 0:06:23
2020-01-15T19:17:18.9292191Z == clock drift check ==
2020-01-15T19:17:18.9312036Z   local time: Wed Jan 15 19:17:18 UTC 2020
2020-01-15T19:17:18.9312036Z   local time: Wed Jan 15 19:17:18 UTC 2020
2020-01-15T19:17:19.2228378Z   network time: Wed, 15 Jan 2020 19:17:19 GMT
2020-01-15T19:17:19.2230174Z == end clock drift check ==
2020-01-15T19:17:19.5603280Z 
2020-01-15T19:17:19.5708877Z ##[error]Bash exited with code '1'.
2020-01-15T19:17:19.5762069Z ##[section]Starting: Checkout
2020-01-15T19:17:19.5764341Z ==============================================================================
2020-01-15T19:17:19.5764406Z Task         : Get sources
2020-01-15T19:17:19.5764460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

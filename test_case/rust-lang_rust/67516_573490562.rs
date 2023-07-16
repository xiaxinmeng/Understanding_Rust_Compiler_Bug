plain
2020-01-13T02:23:53.9894103Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T02:23:53.9905713Z ##[command]git config gc.auto 0
2020-01-13T02:23:53.9908079Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T02:23:53.9910118Z ##[command]git config --get-all http.proxy
2020-01-13T02:23:53.9915568Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67516/merge:refs/remotes/pull/67516/merge
---
2020-01-13T02:31:55.9129406Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-01-13T02:31:56.6841119Z error[E0599]: no method named `is_sanitizer_runtime` found for type `rustc::ty::TyCtxt<'_>` in the current scope
2020-01-13T02:31:56.6841585Z    --> src/librustc_codegen_ssa/base.rs:842:20
2020-01-13T02:31:56.6841815Z     |
2020-01-13T02:31:56.6842914Z 842 |             if tcx.is_sanitizer_runtime(cnum) {
2020-01-13T02:31:56.6843363Z 
2020-01-13T02:31:56.6888873Z error[E0609]: no field `sanitizer_runtime` on type `CrateInfo`
2020-01-13T02:31:56.6889210Z    --> src/librustc_codegen_ssa/base.rs:843:22
2020-01-13T02:31:56.6889470Z     |
2020-01-13T02:31:56.6889470Z     |
2020-01-13T02:31:56.6890465Z 843 |                 info.sanitizer_runtime = Some(cnum.into());
2020-01-13T02:31:56.6891113Z     |
2020-01-13T02:31:56.6891113Z     |
2020-01-13T02:31:56.6891485Z     = note: available fields are: `panic_runtime`, `compiler_builtins`, `profiler_runtime`, `is_no_builtins`, `native_libraries` ... and 9 others
2020-01-13T02:31:57.5292904Z error: aborting due to 2 previous errors
2020-01-13T02:31:57.5293085Z 
2020-01-13T02:31:57.5293466Z Some errors have detailed explanations: E0599, E0609.
2020-01-13T02:31:57.5293764Z For more information about an error, try `rustc --explain E0599`.
2020-01-13T02:31:57.5293764Z For more information about an error, try `rustc --explain E0599`.
2020-01-13T02:31:57.5454075Z error: could not compile `rustc_codegen_ssa`.
2020-01-13T02:31:57.5454188Z 
2020-01-13T02:31:57.5454451Z To learn more, run the command again with --verbose.
2020-01-13T02:31:57.5504566Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-13T02:31:57.5511974Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-13T02:31:57.5512273Z Build completed unsuccessfully in 0:05:43
2020-01-13T02:31:57.5567256Z == clock drift check ==
2020-01-13T02:31:57.5578347Z   local time: Mon Jan 13 02:31:57 UTC 2020
2020-01-13T02:31:57.5578347Z   local time: Mon Jan 13 02:31:57 UTC 2020
2020-01-13T02:31:57.8208071Z   network time: Mon, 13 Jan 2020 02:31:57 GMT
2020-01-13T02:31:57.8208948Z == end clock drift check ==
2020-01-13T02:31:58.2242090Z 
2020-01-13T02:31:58.2349815Z ##[error]Bash exited with code '1'.
2020-01-13T02:31:58.2383815Z ##[section]Starting: Checkout
2020-01-13T02:31:58.2385471Z ==============================================================================
2020-01-13T02:31:58.2385541Z Task         : Get sources
2020-01-13T02:31:58.2385588Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2020-03-17T05:46:50.6060550Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-17T05:46:51.4434494Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-03-17T05:46:54.1926646Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-03-17T05:46:54.4110710Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-03-17T05:46:54.6983885Z error[E0432]: unresolved import `rustc_infer::infer::SuppressRegionErrors`
2020-03-17T05:46:54.6984835Z   --> src/librustc_typeck/impl_wf_check/min_specialization.rs:78:37
2020-03-17T05:46:54.6985414Z    |
2020-03-17T05:46:54.6986140Z 78 | use rustc_infer::infer::{InferCtxt, SuppressRegionErrors, TyCtxtInferExt};
2020-03-17T05:46:54.6987442Z    |                                     ^^^^^^^^^^^^^^^^^^^^ no `SuppressRegionErrors` in `infer`
2020-03-17T05:46:56.9849473Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-03-17T05:46:57.5806247Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-03-17T05:46:58.8582303Z error: aborting due to previous error
2020-03-17T05:46:58.8583283Z 
2020-03-17T05:46:58.8583283Z 
2020-03-17T05:46:58.8584179Z For more information about this error, try `rustc --explain E0432`.
2020-03-17T05:46:58.8644608Z error: could not compile `rustc_typeck`.
2020-03-17T05:46:58.8645192Z 
2020-03-17T05:46:58.8646705Z To learn more, run the command again with --verbose.
2020-03-17T05:46:58.8647991Z warning: build failed, waiting for other jobs to finish...
2020-03-17T05:47:00.4725334Z error: build failed
2020-03-17T05:47:00.4753945Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-17T05:47:00.4769348Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-17T05:47:00.4769771Z Build completed unsuccessfully in 0:04:36
2020-03-17T05:47:00.4822578Z == clock drift check ==
2020-03-17T05:47:00.4843348Z   local time: Tue Mar 17 05:47:00 UTC 2020
2020-03-17T05:47:00.4843348Z   local time: Tue Mar 17 05:47:00 UTC 2020
2020-03-17T05:47:00.6043908Z   network time: Tue, 17 Mar 2020 05:47:00 GMT
2020-03-17T05:47:00.6048073Z == end clock drift check ==
2020-03-17T05:47:01.3065627Z 
2020-03-17T05:47:01.3135484Z ##[error]Bash exited with code '1'.
2020-03-17T05:47:01.3203306Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-17T05:47:01.3208765Z ==============================================================================
2020-03-17T05:47:01.3209166Z Task         : Get sources
2020-03-17T05:47:01.3209556Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

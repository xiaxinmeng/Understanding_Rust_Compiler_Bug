plain
2020-01-12T14:06:40.2569637Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T14:06:40.2578742Z ##[command]git config gc.auto 0
2020-01-12T14:06:40.2581942Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T14:06:40.2583489Z ##[command]git config --get-all http.proxy
2020-01-12T14:06:40.2587033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67668/merge:refs/remotes/pull/67668/merge
---
2020-01-12T14:14:37.4337641Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-01-12T14:14:37.8217165Z error[E0261]: use of undeclared lifetime name `'pat`
2020-01-12T14:14:37.8218280Z    --> src/librustc_mir_build/build/matches/mod.rs:900:42
2020-01-12T14:14:37.8218767Z     |
2020-01-12T14:14:37.8219273Z 900 |         candidates: &mut [&mut Candidate<'pat, 'tcx>],
2020-01-12T14:14:37.8220037Z     |                                          ^^^^ undeclared lifetime
2020-01-12T14:14:37.8245981Z error[E0261]: use of undeclared lifetime name `'pat`
2020-01-12T14:14:37.8246633Z     --> src/librustc_mir_build/build/matches/mod.rs:1185:35
2020-01-12T14:14:37.8247125Z      |
2020-01-12T14:14:37.8247125Z      |
2020-01-12T14:14:37.8247635Z 1185 |         candidate: &mut Candidate<'pat, 'tcx>,
2020-01-12T14:14:37.8248160Z      |                                   ^^^^ undeclared lifetime
2020-01-12T14:14:37.8695054Z error: aborting due to 2 previous errors
2020-01-12T14:14:37.8698259Z 
2020-01-12T14:14:37.8706460Z For more information about this error, try `rustc --explain E0261`.
2020-01-12T14:14:37.8765093Z error: could not compile `rustc_mir_build`.
2020-01-12T14:14:37.8765093Z error: could not compile `rustc_mir_build`.
2020-01-12T14:14:37.8790426Z warning: build failed, waiting for other jobs to finish...
2020-01-12T14:14:49.5083161Z error: build failed
2020-01-12T14:14:49.5103001Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-12T14:14:49.5118245Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-12T14:14:49.5118592Z Build completed unsuccessfully in 0:05:24
2020-01-12T14:14:49.5170182Z == clock drift check ==
2020-01-12T14:14:49.5178761Z   local time: Sun Jan 12 14:14:49 UTC 2020
2020-01-12T14:14:49.5178761Z   local time: Sun Jan 12 14:14:49 UTC 2020
2020-01-12T14:14:49.5455355Z   network time: Sun, 12 Jan 2020 14:14:49 GMT
2020-01-12T14:14:49.5462329Z == end clock drift check ==
2020-01-12T14:14:49.9643240Z 
2020-01-12T14:14:49.9728652Z ##[error]Bash exited with code '1'.
2020-01-12T14:14:49.9756868Z ##[section]Starting: Checkout
2020-01-12T14:14:49.9758325Z ==============================================================================
2020-01-12T14:14:49.9758374Z Task         : Get sources
2020-01-12T14:14:49.9758428Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2020-01-12T19:46:43.9613583Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T19:46:43.9689217Z ##[command]git config gc.auto 0
2020-01-12T19:46:43.9768314Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T19:46:43.9824676Z ##[command]git config --get-all http.proxy
2020-01-12T19:46:43.9967499Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67668/merge:refs/remotes/pull/67668/merge
---
2020-01-12T19:54:48.9636559Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-01-12T19:54:49.1737023Z error: lifetime arguments must be declared prior to type arguments
2020-01-12T19:54:49.1737391Z    --> src/librustc_mir_build/build/matches/mod.rs:900:45
2020-01-12T19:54:49.1737616Z     |
2020-01-12T19:54:49.1737938Z 900 |         candidates: &mut [&mut Candidate<_, 'tcx>],
2020-01-12T19:54:49.1738337Z 
2020-01-12T19:54:49.4282720Z error[E0107]: wrong number of lifetime arguments: expected 2, found 1
2020-01-12T19:54:49.4283103Z    --> src/librustc_mir_build/build/matches/mod.rs:900:32
2020-01-12T19:54:49.4283407Z     |
2020-01-12T19:54:49.4283407Z     |
2020-01-12T19:54:49.4283705Z 900 |         candidates: &mut [&mut Candidate<_, 'tcx>],
2020-01-12T19:54:49.4284067Z     |                                ^^^^^^^^^^^^^^^^^^ expected 2 lifetime arguments
2020-01-12T19:54:49.4295465Z error[E0107]: wrong number of type arguments: expected 0, found 1
2020-01-12T19:54:49.4295842Z    --> src/librustc_mir_build/build/matches/mod.rs:900:45
2020-01-12T19:54:49.4296250Z     |
2020-01-12T19:54:49.4296250Z     |
2020-01-12T19:54:49.4296605Z 900 |         candidates: &mut [&mut Candidate<_, 'tcx>],
2020-01-12T19:54:49.4297028Z 
2020-01-12T19:54:49.4492573Z error: aborting due to 3 previous errors
2020-01-12T19:54:49.4492669Z 
2020-01-12T19:54:49.4496488Z For more information about this error, try `rustc --explain E0107`.
2020-01-12T19:54:49.4496488Z For more information about this error, try `rustc --explain E0107`.
2020-01-12T19:54:49.4538145Z error: could not compile `rustc_mir_build`.
2020-01-12T19:54:49.4545579Z warning: build failed, waiting for other jobs to finish...
2020-01-12T19:55:05.2323091Z error: build failed
2020-01-12T19:55:05.2350567Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-12T19:55:05.2361954Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-12T19:55:05.2362428Z Build completed unsuccessfully in 0:05:36
2020-01-12T19:55:05.2412385Z == clock drift check ==
2020-01-12T19:55:05.2432083Z   local time: Sun Jan 12 19:55:05 UTC 2020
2020-01-12T19:55:05.2432083Z   local time: Sun Jan 12 19:55:05 UTC 2020
2020-01-12T19:55:05.5292024Z   network time: Sun, 12 Jan 2020 19:55:05 GMT
2020-01-12T19:55:05.5293239Z == end clock drift check ==
2020-01-12T19:55:05.8995324Z 
2020-01-12T19:55:05.9101321Z ##[error]Bash exited with code '1'.
2020-01-12T19:55:05.9128729Z ##[section]Starting: Checkout
2020-01-12T19:55:05.9130371Z ==============================================================================
2020-01-12T19:55:05.9130425Z Task         : Get sources
2020-01-12T19:55:05.9130472Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

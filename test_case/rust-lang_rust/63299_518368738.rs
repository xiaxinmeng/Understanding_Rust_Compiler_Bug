plain
2019-08-05T19:12:11.2559602Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T19:12:11.2763586Z ##[command]git config gc.auto 0
2019-08-05T19:12:11.2857584Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T19:12:11.2906273Z ##[command]git config --get-all http.proxy
2019-08-05T19:12:11.3093701Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63299/merge:refs/remotes/pull/63299/merge
---
2019-08-05T19:12:45.7829684Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T19:12:45.7829939Z 
2019-08-05T19:12:45.7830218Z   git checkout -b <new-branch-name>
2019-08-05T19:12:45.7830252Z 
2019-08-05T19:12:45.7830325Z HEAD is now at a7b2f060d Merge d04f1c8ffdae3aea988239007da26bbaf8815056 into f6ecdc2f61b96de199be956cad853a7c02bcfb58
2019-08-05T19:12:45.7998421Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T19:12:45.8001559Z ==============================================================================
2019-08-05T19:12:45.8001620Z Task         : Bash
2019-08-05T19:12:45.8001666Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T19:21:37.9596068Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
2019-08-05T19:21:38.2429057Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-08-05T19:21:38.8317628Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-08-05T19:21:43.6397042Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2019-08-05T19:21:46.3891669Z error[E0507]: cannot move out of `*place.projection` which is behind a shared reference
2019-08-05T19:21:46.3892123Z    --> src/librustc_mir/transform/qualify_consts.rs:187:20
2019-08-05T19:21:46.3892391Z     |
2019-08-05T19:21:46.3892737Z 187 |         let proj = place.projection.unwrap();
2019-08-05T19:21:46.3893344Z     |                    |
2019-08-05T19:21:46.3893344Z     |                    |
2019-08-05T19:21:46.3893834Z     |                    move occurs because `*place.projection` has type `std::option::Option<std::boxed::Box<rustc::mir::Projection<'_>>>`, which does not implement the `Copy` trait
2019-08-05T19:21:46.3894250Z     |                    help: consider borrowing the `Option`'s content: `place.projection.as_ref()`
2019-08-05T19:21:46.3894333Z 
2019-08-05T19:21:46.4028983Z error[E0507]: cannot move out of `*place.projection` which is behind a shared reference
2019-08-05T19:21:46.4029381Z    --> src/librustc_mir/transform/qualify_consts.rs:453:20
2019-08-05T19:21:46.4029690Z     |
2019-08-05T19:21:46.4030014Z 453 |         let proj = place.projection.unwrap();
2019-08-05T19:21:46.4030650Z     |                    |
2019-08-05T19:21:46.4030650Z     |                    |
2019-08-05T19:21:46.4031403Z     |                    move occurs because `*place.projection` has type `std::option::Option<std::boxed::Box<rustc::mir::Projection<'_>>>`, which does not implement the `Copy` trait
2019-08-05T19:21:46.4031850Z     |                    help: consider borrowing the `Option`'s content: `place.projection.as_ref()`
2019-08-05T19:21:49.4923809Z error: aborting due to 2 previous errors
2019-08-05T19:21:49.4924684Z 
2019-08-05T19:21:49.4925441Z For more information about this error, try `rustc --explain E0507`.
2019-08-05T19:21:49.6224067Z error: Could not compile `rustc_mir`.
2019-08-05T19:21:49.6224067Z error: Could not compile `rustc_mir`.
2019-08-05T19:21:49.6224182Z 
2019-08-05T19:21:49.6224505Z To learn more, run the command again with --verbose.
2019-08-05T19:21:49.6254666Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-05T19:21:49.6255086Z expected success, got: exit code: 101
2019-08-05T19:21:49.6273287Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-05T19:21:49.6273530Z Build completed unsuccessfully in 0:06:11
2019-08-05T19:21:50.8624965Z ##[error]Bash exited with code '1'.
2019-08-05T19:21:50.8660671Z ##[section]Starting: Checkout
2019-08-05T19:21:50.8662493Z ==============================================================================
2019-08-05T19:21:50.8662806Z Task         : Get sources
2019-08-05T19:21:50.8662881Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-08-08T21:13:17.8422697Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T21:13:17.8612219Z ##[command]git config gc.auto 0
2019-08-08T21:13:17.8699249Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T21:13:17.8766088Z ##[command]git config --get-all http.proxy
2019-08-08T21:13:17.8920462Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-08-08T21:13:53.9270645Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T21:13:53.9270678Z 
2019-08-08T21:13:53.9270896Z   git checkout -b <new-branch-name>
2019-08-08T21:13:53.9270925Z 
2019-08-08T21:13:53.9270991Z HEAD is now at cf0b4ce86 Merge 5d5681a30965400cc268b273f76e259d458e05ba into 2d1a551e144335e0d60a637d12f410cf65849876
2019-08-08T21:13:53.9417206Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T21:13:53.9421428Z ==============================================================================
2019-08-08T21:13:53.9421493Z Task         : Bash
2019-08-08T21:13:53.9421560Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T21:22:15.5885980Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-08-08T21:22:16.3187782Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-08T21:22:16.9424819Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
2019-08-08T21:22:17.2411887Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-08T21:22:19.6860427Z error[E0507]: cannot move out of index of `std::vec::Vec<rustc::mir::Operand<'_>>`
2019-08-08T21:22:19.6868553Z    --> src/librustc_mir/transform/rustc_peek.rs:209:80
2019-08-08T21:22:19.6868828Z     |
2019-08-08T21:22:19.6869171Z 209 |                 if let mir::Operand::Copy(place) | mir::Operand::Move(place) = args[0] {
2019-08-08T21:22:19.6869611Z     |                                           -----                                ^^^^^^^ help: consider borrowing here: `&args[0]`
2019-08-08T21:22:19.6870203Z     |                                           data moved here
2019-08-08T21:22:19.6870203Z     |                                           data moved here
2019-08-08T21:22:19.6870580Z     |                                           move occurs because `place` has type `rustc::mir::Place<'_>`, which does not implement the `Copy` trait
2019-08-08T21:22:19.9273395Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-08-08T21:22:20.4955333Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-08-08T21:22:22.9472400Z error: aborting due to previous error
2019-08-08T21:22:22.9472506Z 
---
2019-08-08T21:22:25.1201217Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-08T21:22:25.1202158Z expected success, got: exit code: 101
2019-08-08T21:22:25.1214139Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-08T21:22:25.1214596Z Build completed unsuccessfully in 0:05:42
2019-08-08T21:22:26.3808428Z ##[error]Bash exited with code '1'.
2019-08-08T21:22:26.3839386Z ##[section]Starting: Checkout
2019-08-08T21:22:26.3840945Z ==============================================================================
2019-08-08T21:22:26.3840993Z Task         : Get sources
2019-08-08T21:22:26.3841035Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

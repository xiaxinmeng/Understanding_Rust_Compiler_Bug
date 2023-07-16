plain
2019-07-24T09:04:37.7228974Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T09:04:37.7418707Z ##[command]git config gc.auto 0
2019-07-24T09:04:37.7495284Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T09:04:37.7599335Z ##[command]git config --get-all http.proxy
2019-07-24T09:04:37.7690446Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62927/merge:refs/remotes/pull/62927/merge
---
2019-07-24T09:05:11.7132651Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T09:05:11.7132727Z 
2019-07-24T09:05:11.7132968Z   git checkout -b <new-branch-name>
2019-07-24T09:05:11.7132998Z 
2019-07-24T09:05:11.7133073Z HEAD is now at 6db84665c Merge 495f9509fe65794384ad62bba90ce742368fdc23 into a7f28678bbf4e16893bb6a718e427504167a9494
2019-07-24T09:05:11.7299163Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T09:05:11.7302297Z ==============================================================================
2019-07-24T09:05:11.7302357Z Task         : Bash
2019-07-24T09:05:11.7302407Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T09:14:16.7554989Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-07-24T09:14:17.1213865Z error: unused import: `InterpError`
2019-07-24T09:14:17.1220795Z  --> src/librustc_codegen_ssa/mir/block.rs:5:29
2019-07-24T09:14:17.1225762Z   |
2019-07-24T09:14:17.1231208Z 5 | use rustc::mir::interpret::{InterpError, PanicMessage};
2019-07-24T09:14:17.1242140Z   |
2019-07-24T09:14:17.1242140Z   |
2019-07-24T09:14:17.1247881Z   = note: `-D unused-imports` implied by `-D warnings`
2019-07-24T09:14:19.7302660Z error: aborting due to previous error
2019-07-24T09:14:19.7302777Z 
2019-07-24T09:14:19.7526253Z error: Could not compile `rustc_codegen_ssa`.
2019-07-24T09:14:19.7546973Z warning: build failed, waiting for other jobs to finish...
2019-07-24T09:14:19.7546973Z warning: build failed, waiting for other jobs to finish...
2019-07-24T09:14:19.9385611Z error: build failed
2019-07-24T09:14:19.9416351Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-24T09:14:19.9431823Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-24T09:14:19.9431911Z Build completed unsuccessfully in 0:05:58
2019-07-24T09:14:19.9431911Z Build completed unsuccessfully in 0:05:58
2019-07-24T09:14:21.1852857Z ##[error]Bash exited with code '1'.
2019-07-24T09:14:21.1886746Z ##[section]Starting: Checkout
2019-07-24T09:14:21.1888559Z ==============================================================================
2019-07-24T09:14:21.1888616Z Task         : Get sources
2019-07-24T09:14:21.1888669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

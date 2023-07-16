plain
2019-08-08T20:36:23.1361140Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T20:36:23.1545378Z ##[command]git config gc.auto 0
2019-08-08T20:36:23.1615774Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T20:36:23.1677889Z ##[command]git config --get-all http.proxy
2019-08-08T20:36:23.1814961Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-08-08T20:37:01.5752993Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T20:37:01.5753025Z 
2019-08-08T20:37:01.5753251Z   git checkout -b <new-branch-name>
2019-08-08T20:37:01.5753282Z 
2019-08-08T20:37:01.5753329Z HEAD is now at 0d22804b7 Merge fe93ca4d10e620566c9a69fbdfdce43282019895 into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T20:37:01.5913371Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T20:37:01.5917219Z ==============================================================================
2019-08-08T20:37:01.5917276Z Task         : Bash
2019-08-08T20:37:01.5917319Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T20:45:34.1602113Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-08-08T20:45:35.4413413Z error: unused import: `ProjectionElem`
2019-08-08T20:45:35.4413954Z  --> src/librustc_mir/borrow_check/path_utils.rs:5:76
2019-08-08T20:45:35.4414241Z   |
2019-08-08T20:45:35.4419570Z 5 | use rustc::mir::{BasicBlock, BorrowKind, Location, Body, Place, PlaceBase, ProjectionElem};
2019-08-08T20:45:35.4420208Z   |
2019-08-08T20:45:35.4420807Z   = note: `-D unused-imports` implied by `-D warnings`
2019-08-08T20:45:35.4420843Z 
2019-08-08T20:45:37.5866529Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
---
2019-08-08T20:45:44.9935950Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-08T20:45:44.9936577Z expected success, got: exit code: 101
2019-08-08T20:45:44.9944352Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-08T20:45:44.9944736Z Build completed unsuccessfully in 0:05:43
2019-08-08T20:45:46.2131573Z ##[error]Bash exited with code '1'.
2019-08-08T20:45:46.2162316Z ##[section]Starting: Checkout
2019-08-08T20:45:46.2164399Z ==============================================================================
2019-08-08T20:45:46.2164458Z Task         : Get sources
2019-08-08T20:45:46.2164508Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

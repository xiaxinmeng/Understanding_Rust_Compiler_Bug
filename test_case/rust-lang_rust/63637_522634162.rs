plain
2019-08-19T15:33:10.1720776Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T15:33:10.1925541Z ##[command]git config gc.auto 0
2019-08-19T15:33:10.1984105Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T15:33:10.2048492Z ##[command]git config --get-all http.proxy
2019-08-19T15:33:10.2185213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63637/merge:refs/remotes/pull/63637/merge
---
2019-08-19T15:33:45.1773277Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T15:33:45.1773939Z 
2019-08-19T15:33:45.1774742Z   git checkout -b <new-branch-name>
2019-08-19T15:33:45.1775415Z 
2019-08-19T15:33:45.1775619Z HEAD is now at e8c5ff162 Merge 92bec03c84da57fa71c6b94fb60178139e813734 into cdff9189556bb7de2b9a8a72344c9d8ec6099fcd
2019-08-19T15:33:45.1926016Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T15:33:45.1928871Z ==============================================================================
2019-08-19T15:33:45.1928948Z Task         : Bash
2019-08-19T15:33:45.1928995Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T15:39:25.1095974Z     Checking hashbrown v0.5.0
2019-08-19T15:39:30.8348220Z     Checking rustc-std-workspace-std v1.0.0 (/checkout/src/tools/rustc-std-workspace-std)
2019-08-19T15:39:30.8371885Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-08-19T15:39:30.8866066Z     Checking term v0.0.0 (/checkout/src/libterm)
2019-08-19T15:39:30.9421522Z error[E0463]: can't find crate for `core`
2019-08-19T15:39:30.9422814Z  --> <::alloc::macros::format macros>:2:31
2019-08-19T15:39:30.9423375Z   |
2019-08-19T15:39:30.9423935Z 2 | ($ crate :: fmt :: format (:: core :: format_args ! ($ ($ arg) *)))
2019-08-19T15:39:30.9427957Z 
2019-08-19T15:39:30.9444260Z error: aborting due to previous error
2019-08-19T15:39:30.9448265Z 
2019-08-19T15:39:30.9458693Z For more information about this error, try `rustc --explain E0463`.
2019-08-19T15:39:30.9458693Z For more information about this error, try `rustc --explain E0463`.
2019-08-19T15:39:30.9502189Z error: Could not compile `term`.
2019-08-19T15:39:33.7339610Z error: build failed
2019-08-19T15:39:33.7339610Z error: build failed
2019-08-19T15:39:33.7370060Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-19T15:39:33.7377138Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-19T15:39:33.7377544Z Build completed unsuccessfully in 0:02:58
2019-08-19T15:39:33.7432149Z == clock drift check ==
2019-08-19T15:39:33.7445005Z   local time: Mon Aug 19 15:39:33 UTC 2019
2019-08-19T15:39:33.7445005Z   local time: Mon Aug 19 15:39:33 UTC 2019
2019-08-19T15:39:33.8943012Z   network time: Mon, 19 Aug 2019 15:39:33 GMT
2019-08-19T15:39:33.8946975Z == end clock drift check ==
2019-08-19T15:39:39.8796260Z ##[error]Bash exited with code '1'.
2019-08-19T15:39:39.8833564Z ##[section]Starting: Checkout
2019-08-19T15:39:39.8835607Z ==============================================================================
2019-08-19T15:39:39.8835664Z Task         : Get sources
2019-08-19T15:39:39.8835730Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

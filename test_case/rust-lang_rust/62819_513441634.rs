plain
2019-07-20T06:20:51.2814242Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T06:20:51.2987049Z ##[command]git config gc.auto 0
2019-07-20T06:20:51.3064125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T06:20:51.3123775Z ##[command]git config --get-all http.proxy
2019-07-20T06:20:51.3272688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62819/merge:refs/remotes/pull/62819/merge
---
2019-07-20T06:21:24.9015811Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T06:21:24.9016356Z 
2019-07-20T06:21:24.9016905Z   git checkout -b <new-branch-name>
2019-07-20T06:21:24.9017190Z 
2019-07-20T06:21:24.9017462Z HEAD is now at c287158e4 Merge 3f05394f6599bb3de1e0fd746a2c00a2d4073c16 into e9d22273283dce210b26362aa0dcc3fc10bf7e81
2019-07-20T06:21:24.9169771Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T06:21:24.9173529Z ==============================================================================
2019-07-20T06:21:24.9173597Z Task         : Bash
2019-07-20T06:21:24.9173668Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T06:27:20.9554623Z     Checking term v0.0.0 (/checkout/src/libterm)
2019-07-20T06:27:21.0641105Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-20T06:27:21.3535377Z     Checking getopts v0.2.19
2019-07-20T06:27:24.2694006Z     Checking test v0.0.0 (/checkout/src/libtest)
2019-07-20T06:27:24.9811828Z error[E0636]: the feature `rustc_private` has already been declared
2019-07-20T06:27:24.9812216Z   --> src/libtest/lib.rs:32:12
2019-07-20T06:27:24.9812471Z    |
2019-07-20T06:27:24.9812794Z 32 | #![feature(rustc_private)]
2019-07-20T06:27:24.9813189Z 
2019-07-20T06:27:24.9937686Z error: aborting due to previous error
2019-07-20T06:27:24.9937814Z 
2019-07-20T06:27:24.9938144Z For more information about this error, try `rustc --explain E0636`.
2019-07-20T06:27:24.9938144Z For more information about this error, try `rustc --explain E0636`.
2019-07-20T06:27:25.0103158Z error: Could not compile `test`.
2019-07-20T06:27:25.0103296Z 
2019-07-20T06:27:25.0103564Z To learn more, run the command again with --verbose.
2019-07-20T06:27:25.0121386Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-07-20T06:27:25.0121571Z expected success, got: exit code: 101
2019-07-20T06:27:25.0132425Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-20T06:27:25.0132580Z Build completed unsuccessfully in 0:02:58
2019-07-20T06:27:27.0767439Z ##[error]Bash exited with code '1'.
2019-07-20T06:27:27.0802158Z ##[section]Starting: Checkout
2019-07-20T06:27:27.0804299Z ==============================================================================
2019-07-20T06:27:27.0804378Z Task         : Get sources
2019-07-20T06:27:27.0804434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

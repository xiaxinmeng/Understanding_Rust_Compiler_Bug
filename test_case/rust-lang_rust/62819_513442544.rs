plain
2019-07-20T06:38:17.8343577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T06:38:17.8519663Z ##[command]git config gc.auto 0
2019-07-20T06:38:17.8582682Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T06:38:17.8632619Z ##[command]git config --get-all http.proxy
2019-07-20T06:38:17.8750341Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62819/merge:refs/remotes/pull/62819/merge
---
2019-07-20T06:38:51.6387780Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T06:38:51.6387989Z 
2019-07-20T06:38:51.6388355Z   git checkout -b <new-branch-name>
2019-07-20T06:38:51.6388572Z 
2019-07-20T06:38:51.6388758Z HEAD is now at 51d791d4c Merge b8d4c7762099a3cce0a1046db0556a848f9fdf9d into e9d22273283dce210b26362aa0dcc3fc10bf7e81
2019-07-20T06:38:51.6508838Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T06:38:51.6511640Z ==============================================================================
2019-07-20T06:38:51.6511700Z Task         : Bash
2019-07-20T06:38:51.6511747Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T06:44:12.3601960Z     Checking test v0.0.0 (/checkout/src/libtest)
2019-07-20T06:44:12.3913690Z error: unexpected close delimiter: `]`
2019-07-20T06:44:12.3920123Z   --> src/libtest/lib.rs:25:33
2019-07-20T06:44:12.3922337Z    |
2019-07-20T06:44:12.3925900Z 25 | #![feature(libc, rustc_private))]
2019-07-20T06:44:12.3927295Z    |                                 ^ unexpected close delimiter
2019-07-20T06:44:12.3930468Z error: incorrect close delimiter: `)`
2019-07-20T06:44:12.3932746Z   --> src/libtest/lib.rs:25:32
2019-07-20T06:44:12.3934768Z    |
2019-07-20T06:44:12.3934768Z    |
2019-07-20T06:44:12.3938527Z 25 | #![feature(libc, rustc_private))]
2019-07-20T06:44:12.3942284Z    |   - un-closed delimiter        ^ incorrect close delimiter
2019-07-20T06:44:12.3947000Z error: aborting due to 2 previous errors
2019-07-20T06:44:12.3950239Z 
2019-07-20T06:44:12.3992398Z error: Could not compile `test`.
2019-07-20T06:44:12.3992718Z 
2019-07-20T06:44:12.3992718Z 
2019-07-20T06:44:12.3993180Z To learn more, run the command again with --verbose.
2019-07-20T06:44:12.4011801Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-07-20T06:44:12.4011945Z expected success, got: exit code: 101
2019-07-20T06:44:12.4022657Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-20T06:44:12.4022960Z Build completed unsuccessfully in 0:02:22
2019-07-20T06:44:14.4302616Z ##[error]Bash exited with code '1'.
2019-07-20T06:44:14.4333794Z ##[section]Starting: Checkout
2019-07-20T06:44:14.4335297Z ==============================================================================
2019-07-20T06:44:14.4335339Z Task         : Get sources
2019-07-20T06:44:14.4335375Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

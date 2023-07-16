plain
2019-07-21T14:32:57.3046273Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T14:32:57.3246464Z ##[command]git config gc.auto 0
2019-07-21T14:32:57.3324540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T14:32:57.3383647Z ##[command]git config --get-all http.proxy
2019-07-21T14:32:57.9923519Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62829/merge:refs/remotes/pull/62829/merge
---
2019-07-21T14:33:31.1171855Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T14:33:31.1172027Z 
2019-07-21T14:33:31.1172397Z   git checkout -b <new-branch-name>
2019-07-21T14:33:31.1172538Z 
2019-07-21T14:33:31.1173980Z HEAD is now at aaf7f6fb6 Merge 68b3c9cb8e2374dd827d60f65f89c333d7b7d245 into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T14:33:31.1309625Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T14:33:31.1312929Z ==============================================================================
2019-07-21T14:33:31.1312980Z Task         : Bash
2019-07-21T14:33:31.1313020Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T14:36:44.5600673Z ##################                                                        25.2%
2019-07-21T14:36:44.5603574Z ######################################################################## 100.0%
2019-07-21T14:36:44.9896506Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-21T14:36:45.0780225Z     Updating crates.io index
2019-07-21T14:37:05.7808276Z     Updating git repository `https://github.com/gnzlbg/libtest`
---
2019-07-21T14:39:07.0042237Z     Checking unicode-width v0.1.5
2019-07-21T14:39:07.1126887Z     Checking termcolor v1.0.4
2019-07-21T14:39:07.4509384Z     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
2019-07-21T14:39:07.8544765Z     Checking getopts v0.2.19
2019-07-21T14:39:09.1675505Z     Checking libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#1f13f7dd)
2019-07-21T14:39:10.6369206Z error: unused extern crate
2019-07-21T14:39:10.6378536Z   --> src/libtest/lib.rs:19:1
2019-07-21T14:39:10.6385457Z    |
2019-07-21T14:39:10.6392939Z 19 | extern crate libtest;
2019-07-21T14:39:10.6392939Z 19 | extern crate libtest;
2019-07-21T14:39:10.6393375Z    | ^^^^^^^^^^^^^^^^^^^^^ help: remove it
2019-07-21T14:39:10.6393654Z    |
2019-07-21T14:39:10.6393964Z    = note: `-D unused-extern-crates` implied by `-D rust-2018-idioms`
2019-07-21T14:39:10.6394276Z error: aborting due to previous error
2019-07-21T14:39:10.6394314Z 
2019-07-21T14:39:10.6443438Z error: Could not compile `test`.
2019-07-21T14:39:10.6443521Z 
2019-07-21T14:39:10.6443521Z 
2019-07-21T14:39:10.6443817Z To learn more, run the command again with --verbose.
2019-07-21T14:39:10.6466304Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
2019-07-21T14:39:10.6466628Z expected success, got: exit code: 101
2019-07-21T14:39:10.6478055Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-21T14:39:10.6478440Z Build completed unsuccessfully in 0:02:43
2019-07-21T14:39:12.9172745Z ##[error]Bash exited with code '1'.
2019-07-21T14:39:12.9205670Z ##[section]Starting: Checkout
2019-07-21T14:39:12.9207363Z ==============================================================================
2019-07-21T14:39:12.9207419Z Task         : Get sources
2019-07-21T14:39:12.9207611Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

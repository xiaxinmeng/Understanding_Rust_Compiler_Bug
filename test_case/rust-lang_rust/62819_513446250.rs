plain
2019-07-20T06:48:21.7817723Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T06:48:21.8022153Z ##[command]git config gc.auto 0
2019-07-20T06:48:21.8108143Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T06:48:21.8169588Z ##[command]git config --get-all http.proxy
2019-07-20T06:48:21.8317208Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62819/merge:refs/remotes/pull/62819/merge
---
2019-07-20T06:48:55.0246163Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T06:48:55.0246194Z 
2019-07-20T06:48:55.0246827Z   git checkout -b <new-branch-name>
2019-07-20T06:48:55.0246865Z 
2019-07-20T06:48:55.0246939Z HEAD is now at 951e597e2 Merge 37edd4d8eb8872a128cec22f6577ca012303a4f8 into e9d22273283dce210b26362aa0dcc3fc10bf7e81
2019-07-20T06:48:55.0395416Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T06:48:55.0398501Z ==============================================================================
2019-07-20T06:48:55.0398563Z Task         : Bash
2019-07-20T06:48:55.0398632Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T07:46:59.2472302Z .................................................................................................... 200/5842
2019-07-20T07:47:03.1405555Z .................................................................................................... 300/5842
2019-07-20T07:47:06.4807077Z .................................................................................................... 400/5842
2019-07-20T07:47:09.8542931Z .................................................................................................... 500/5842
2019-07-20T07:47:13.2724177Z ........................................................................i........................... 600/5842
2019-07-20T07:47:21.3040041Z .................................................................................................... 800/5842
2019-07-20T07:47:26.3963617Z .................................................................................................... 900/5842
2019-07-20T07:47:30.9634089Z ...................................................................................................i 1000/5842
2019-07-20T07:47:30.9634089Z ...................................................................................................i 1000/5842
2019-07-20T07:47:35.9936567Z ...........i........................................................................................ 1100/5842
2019-07-20T07:47:39.6147094Z .............................iiiii.................................................................. 1200/5842
2019-07-20T07:47:44.9836393Z .................................................................................................... 1400/5842
2019-07-20T07:47:47.4559608Z .................................................................................................... 1500/5842
2019-07-20T07:47:50.8807212Z .................................................................................................... 1600/5842
2019-07-20T07:47:53.2425057Z .................................................................................................... 1700/5842
2019-07-20T07:47:53.2425057Z .................................................................................................... 1700/5842
2019-07-20T07:47:56.3644679Z ....................F.........F......................................i.............................. 1800/5842
2019-07-20T07:48:04.2151617Z .................................................................................................... 2000/5842
2019-07-20T07:48:08.0795632Z .................................................................................................... 2100/5842
2019-07-20T07:48:11.4855733Z .................................................................................................... 2200/5842
2019-07-20T07:48:11.4855733Z .................................................................................................... 2200/5842
2019-07-20T07:48:15.0472368Z ....................................................i............................................... 2300/5842
2019-07-20T07:48:24.1020607Z .................................................................................................... 2500/5842
2019-07-20T07:48:27.9035750Z .................................................................................................... 2600/5842
2019-07-20T07:48:32.6184778Z .................................................................................................... 2700/5842
2019-07-20T07:48:36.1576418Z .................................................................................................... 2800/5842
2019-07-20T07:48:36.1576418Z .................................................................................................... 2800/5842
2019-07-20T07:48:40.1749379Z .........F.......................................................................................... 2900/5842
2019-07-20T07:48:45.0252778Z .................................................................................................... 3000/5842
2019-07-20T07:48:49.1318652Z .................................................................................................... 3100/5842
2019-07-20T07:48:54.0135403Z .................................................................................................... 3200/5842
2019-07-20T07:48:57.2376904Z .................................................................................................... 3300/5842
2019-07-20T07:49:00.6257035Z .................................................................................................... 3400/5842
2019-07-20T07:49:05.2680978Z .................................................................................................... 3500/5842
2019-07-20T07:49:08.6065958Z ..................i................................................................................. 3600/5842
2019-07-20T07:49:12.3803983Z ............................................................................................ii...i.. 3700/5842
2019-07-20T07:49:15.9385083Z ii.................................................................................................. 3800/5842
2019-07-20T07:49:23.7882591Z .................................................................................................... 4000/5842
2019-07-20T07:49:23.7882591Z .................................................................................................... 4000/5842
2019-07-20T07:49:27.1241801Z ......ii............................................................................................ 4100/5842
2019-07-20T07:49:28.9569604Z ...........................i........................................................................ 4200/5842
2019-07-20T07:49:30.7362622Z .............................................................................................i...... 4300/5842
2019-07-20T07:49:37.0963142Z .................................................................................................... 4500/5842
2019-07-20T07:49:53.0695298Z .................................................................................................... 4600/5842
2019-07-20T07:49:56.2194693Z .................................................................................................... 4700/5842
2019-07-20T07:49:59.6775653Z .................................................................................................... 4800/5842
---
2019-07-20T07:50:31.3080756Z .................................................................................................... 5400/5842
2019-07-20T07:50:34.9336131Z .................................................................................................... 5500/5842
2019-07-20T07:50:38.7392068Z .................................................................................................... 5600/5842
2019-07-20T07:50:41.5952277Z .................................................................................................... 5700/5842
2019-07-20T07:50:44.2323275Z ..................................................................................i................. 5800/5842
2019-07-20T07:50:45.5865410Z failures:
2019-07-20T07:50:45.5898496Z 
2019-07-20T07:50:45.5899389Z ---- [ui] ui/feature-gates/feature-gate-rustc_private-libc.rs stdout ----
2019-07-20T07:50:45.5899670Z diff of stderr:
2019-07-20T07:50:45.5899670Z diff of stderr:
2019-07-20T07:50:45.5899709Z 
2019-07-20T07:50:45.5899756Z 4 LL |     use libc::*;
2019-07-20T07:50:45.5899827Z 5    |         ^^^^ maybe a missing `extern crate libc;`?
2019-07-20T07:50:45.5899875Z 6 
2019-07-20T07:50:45.5900362Z - error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
2019-07-20T07:50:45.5900787Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-20T07:50:45.5901224Z 8   --> $DIR/feature-gate-rustc_private-libc.rs:2:5
2019-07-20T07:50:45.5901350Z 10 LL |     extern crate libc;
2019-07-20T07:50:45.5901383Z 
2019-07-20T07:50:45.5901428Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-20T07:50:45.5901493Z 12    |
2019-07-20T07:50:45.5901493Z 12    |
2019-07-20T07:50:45.5901749Z -    = help: add #![feature(rustc_private)] to the crate attributes to enable
2019-07-20T07:50:45.5902146Z +    = note: for more information, see ***/issues/27812
2019-07-20T07:50:45.5902233Z +    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-20T07:50:45.5902529Z 15 error: aborting due to 2 previous errors
2019-07-20T07:50:45.5902612Z 16 
2019-07-20T07:50:45.5902642Z 
2019-07-20T07:50:45.5903245Z - Some errors occurred: E0432, E0658.
2019-07-20T07:50:45.5903245Z - Some errors occurred: E0432, E0658.
2019-07-20T07:50:45.5903289Z + Some errors have detailed explanations: E0432, E0658.
2019-07-20T07:50:45.5903496Z 18 For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5903542Z 19 
2019-07-20T07:50:45.5903563Z 
2019-07-20T07:50:45.5903584Z 
2019-07-20T07:50:45.5903639Z The actual stderr differed from the expected stderr.
2019-07-20T07:50:45.5903894Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc/feature-gate-rustc_private-libc.stderr
2019-07-20T07:50:45.5904083Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:50:45.5904315Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-rustc_private-libc.rs`
2019-07-20T07:50:45.5904387Z error: 1 errors occurred comparing output.
2019-07-20T07:50:45.5904440Z status: exit code: 1
2019-07-20T07:50:45.5904440Z status: exit code: 1
2019-07-20T07:50:45.5905221Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-rustc_private-libc/auxiliary" "-A" "unused"
2019-07-20T07:50:45.5905492Z ------------------------------------------
2019-07-20T07:50:45.5905520Z 
2019-07-20T07:50:45.5905704Z ------------------------------------------
2019-07-20T07:50:45.5905743Z stderr:
2019-07-20T07:50:45.5905743Z stderr:
2019-07-20T07:50:45.5905913Z ------------------------------------------
2019-07-20T07:50:45.5905954Z error[E0432]: unresolved import `libc`
2019-07-20T07:50:45.5906171Z   --> /checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs:3:9
2019-07-20T07:50:45.5906214Z    |
2019-07-20T07:50:45.5906255Z LL |     use libc::*; //~ ERROR unresolved import
2019-07-20T07:50:45.5906416Z    |         ^^^^ maybe a missing `extern crate libc;`?
2019-07-20T07:50:45.5906441Z 
2019-07-20T07:50:45.5906768Z error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-20T07:50:45.5907000Z   --> /checkout/src/test/ui/feature-gates/feature-gate-rustc_private-libc.rs:2:5
2019-07-20T07:50:45.5907043Z    |
2019-07-20T07:50:45.5907081Z LL |     extern crate libc; //~ ERROR use of unstable
2019-07-20T07:50:45.5907172Z    |
2019-07-20T07:50:45.5907172Z    |
2019-07-20T07:50:45.5907412Z    = note: for more information, see ***/issues/27812
2019-07-20T07:50:45.5907479Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-20T07:50:45.5907542Z error: aborting due to 2 previous errors
2019-07-20T07:50:45.5907566Z 
2019-07-20T07:50:45.5907620Z Some errors have detailed explanations: E0432, E0658.
2019-07-20T07:50:45.5907830Z For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5907830Z For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5907858Z 
2019-07-20T07:50:45.5908042Z ------------------------------------------
2019-07-20T07:50:45.5908068Z 
2019-07-20T07:50:45.5908089Z 
2019-07-20T07:50:45.5908266Z ---- [ui] ui/feature-gates/feature-gate-test.rs stdout ----
2019-07-20T07:50:45.5908324Z diff of stderr:
2019-07-20T07:50:45.5908348Z 
2019-07-20T07:50:45.5908383Z 4 LL |     use test::*;
2019-07-20T07:50:45.5908422Z 5    |         ^^^^ maybe a missing `extern crate test;`?
2019-07-20T07:50:45.5908474Z 6 
2019-07-20T07:50:45.5908740Z - error[E0658]: use of unstable library feature 'test' (see issue #27812)
2019-07-20T07:50:45.5908964Z + error[E0658]: use of unstable library feature 'test'
2019-07-20T07:50:45.5909641Z 9    |
2019-07-20T07:50:45.5909691Z 10 LL |     extern crate test;
2019-07-20T07:50:45.5909723Z 
2019-07-20T07:50:45.5909795Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-20T07:50:45.5909795Z 11    |     ^^^^^^^^^^^^^^^^^^
2019-07-20T07:50:45.5909842Z 12    |
2019-07-20T07:50:45.5910093Z -    = help: add #![feature(test)] to the crate attributes to enable
2019-07-20T07:50:45.5910412Z +    = note: for more information, see ***/issues/27812
2019-07-20T07:50:45.5910481Z +    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-20T07:50:45.5910603Z 15 error: aborting due to 2 previous errors
2019-07-20T07:50:45.5910652Z 16 
2019-07-20T07:50:45.5910684Z 
2019-07-20T07:50:45.5910954Z - Some errors occurred: E0432, E0658.
2019-07-20T07:50:45.5910954Z - Some errors occurred: E0432, E0658.
2019-07-20T07:50:45.5911039Z + Some errors have detailed explanations: E0432, E0658.
2019-07-20T07:50:45.5911317Z 18 For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5911370Z 19 
2019-07-20T07:50:45.5911420Z 
2019-07-20T07:50:45.5911451Z 
2019-07-20T07:50:45.5911502Z The actual stderr differed from the expected stderr.
2019-07-20T07:50:45.5911836Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test/feature-gate-test.stderr
2019-07-20T07:50:45.5912142Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:50:45.5912433Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-test.rs`
2019-07-20T07:50:45.5912541Z error: 1 errors occurred comparing output.
2019-07-20T07:50:45.5912592Z status: exit code: 1
2019-07-20T07:50:45.5912592Z status: exit code: 1
2019-07-20T07:50:45.5913557Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-test/auxiliary" "-A" "unused"
2019-07-20T07:50:45.5913933Z ------------------------------------------
2019-07-20T07:50:45.5913961Z 
2019-07-20T07:50:45.5914142Z ------------------------------------------
2019-07-20T07:50:45.5914178Z stderr:
2019-07-20T07:50:45.5914178Z stderr:
2019-07-20T07:50:45.5914333Z ------------------------------------------
2019-07-20T07:50:45.5914389Z error[E0432]: unresolved import `test`
2019-07-20T07:50:45.5914570Z   --> /checkout/src/test/ui/feature-gates/feature-gate-test.rs:3:9
2019-07-20T07:50:45.5914609Z    |
2019-07-20T07:50:45.5914664Z LL |     use test::*; //~ ERROR unresolved import
2019-07-20T07:50:45.5914708Z    |         ^^^^ maybe a missing `extern crate test;`?
2019-07-20T07:50:45.5914732Z 
2019-07-20T07:50:45.5914905Z error[E0658]: use of unstable library feature 'test'
2019-07-20T07:50:45.5915142Z    |
2019-07-20T07:50:45.5915142Z    |
2019-07-20T07:50:45.5915176Z LL |     extern crate test; //~ ERROR use of unstable
2019-07-20T07:50:45.5915269Z    |
2019-07-20T07:50:45.5915269Z    |
2019-07-20T07:50:45.5915483Z    = note: for more information, see ***/issues/27812
2019-07-20T07:50:45.5915544Z    = help: add `#![feature(test)]` to the crate attributes to enable
2019-07-20T07:50:45.5915604Z error: aborting due to 2 previous errors
2019-07-20T07:50:45.5915626Z 
2019-07-20T07:50:45.5915678Z Some errors have detailed explanations: E0432, E0658.
2019-07-20T07:50:45.5915869Z For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5915869Z For more information about an error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5915895Z 
2019-07-20T07:50:45.5916142Z ------------------------------------------
2019-07-20T07:50:45.5916173Z 
2019-07-20T07:50:45.5916193Z 
2019-07-20T07:50:45.5916389Z ---- [ui] ui/issues/issue-37887.rs stdout ----
2019-07-20T07:50:45.5916445Z diff of stderr:
2019-07-20T07:50:45.5916469Z 
2019-07-20T07:50:45.5916500Z 4 LL |     use libc::*;
2019-07-20T07:50:45.5916544Z 5    |         ^^^^ maybe a missing `extern crate libc;`?
2019-07-20T07:50:45.5916595Z 6 
2019-07-20T07:50:45.5916884Z - error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead? (see issue #27812)
2019-07-20T07:50:45.5917176Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-20T07:50:45.5917369Z 8   --> $DIR/issue-37887.rs:2:5
2019-07-20T07:50:45.5917445Z 10 LL |     extern crate libc;
2019-07-20T07:50:45.5917487Z 
2019-07-20T07:50:45.5917507Z 
2019-07-20T07:50:45.5917541Z The actual stderr differed from the expected stderr.
2019-07-20T07:50:45.5917541Z The actual stderr differed from the expected stderr.
2019-07-20T07:50:45.5917761Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/issue-37887.stderr
2019-07-20T07:50:45.5917969Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:50:45.5918159Z To only update this specific test, also pass `--test-args issues/issue-37887.rs`
2019-07-20T07:50:45.5918237Z error: 1 errors occurred comparing output.
2019-07-20T07:50:45.5918272Z status: exit code: 1
2019-07-20T07:50:45.5918272Z status: exit code: 1
2019-07-20T07:50:45.5922703Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37887.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37887/auxiliary" "-A" "unused"
2019-07-20T07:50:45.5923382Z ------------------------------------------
2019-07-20T07:50:45.5923549Z 
2019-07-20T07:50:45.5923752Z ------------------------------------------
2019-07-20T07:50:45.5923788Z stderr:
2019-07-20T07:50:45.5923788Z stderr:
2019-07-20T07:50:45.5923945Z ------------------------------------------
2019-07-20T07:50:45.5924184Z error[E0432]: unresolved import `libc`
2019-07-20T07:50:45.5924375Z   --> /checkout/src/test/ui/issues/issue-37887.rs:3:9
2019-07-20T07:50:45.5924416Z    |
2019-07-20T07:50:45.5924472Z LL |     use libc::*; //~ ERROR unresolved import
2019-07-20T07:50:45.5924514Z    |         ^^^^ maybe a missing `extern crate libc;`?
2019-07-20T07:50:45.5924542Z 
2019-07-20T07:50:45.5924868Z error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-20T07:50:45.5925070Z   --> /checkout/src/test/ui/issues/issue-37887.rs:2:5
2019-07-20T07:50:45.5925112Z    |
2019-07-20T07:50:45.5925168Z LL |     extern crate libc; //~ ERROR use of unstable
2019-07-20T07:50:45.5925250Z    |
2019-07-20T07:50:45.5925250Z    |
2019-07-20T07:50:45.5925523Z    = note: for more information, see ***/issues/27812
2019-07-20T07:50:45.5925572Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-20T07:50:45.5925654Z error: aborting due to 2 previous errors
2019-07-20T07:50:45.5925679Z 
2019-07-20T07:50:45.5925716Z Some errors have detailed explanations: E0432, E0658.
2019-07-20T07:50:45.5925927Z For more information about an error, try `rustc --explain E0432`.
---
2019-07-20T07:50:45.5926574Z diff of stderr:
2019-07-20T07:50:45.5926598Z 
2019-07-20T07:50:45.5926775Z - error[E0432]: unresolved import `libtest`
2019-07-20T07:50:45.5926978Z -   --> $DIR/rustc_private-libtest.rs:3:9
2019-07-20T07:50:45.5927158Z + error[E0463]: can't find crate for `libtest`
2019-07-20T07:50:45.5927673Z +   --> $DIR/rustc_private-libtest.rs:2:5
2019-07-20T07:50:45.5927874Z - LL |     use libtest::*;
2019-07-20T07:50:45.5927874Z - LL |     use libtest::*;
2019-07-20T07:50:45.5928049Z -    |         ^^^^^^^ maybe a missing `extern crate libtest;`?
2019-07-20T07:50:45.5928088Z + LL |     extern crate libtest;
2019-07-20T07:50:45.5928307Z 6 
2019-07-20T07:50:45.5928343Z 7 error: aborting due to previous error
2019-07-20T07:50:45.5928390Z 8 
2019-07-20T07:50:45.5928418Z 
2019-07-20T07:50:45.5928418Z 
2019-07-20T07:50:45.5928599Z - For more information about this error, try `rustc --explain E0432`.
2019-07-20T07:50:45.5928781Z + For more information about this error, try `rustc --explain E0463`.
2019-07-20T07:50:45.5928834Z 10 
2019-07-20T07:50:45.5928856Z 
2019-07-20T07:50:45.5928876Z 
2019-07-20T07:50:45.5929091Z The actual stderr differed from the expected stderr.
2019-07-20T07:50:45.5929621Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/rustc_private-libtest.stderr
2019-07-20T07:50:45.5929873Z To update references, rerun the tests and pass the `--bless` flag
2019-07-20T07:50:45.5930133Z To only update this specific test, also pass `--test-args rustc_private-libtest.rs`
2019-07-20T07:50:45.5930236Z error: 1 errors occurred comparing output.
2019-07-20T07:50:45.5930284Z status: exit code: 1
2019-07-20T07:50:45.5930284Z status: exit code: 1
2019-07-20T07:50:45.5931036Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rustc_private-libtest.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc_private-libtest/auxiliary" "-A" "unused"
2019-07-20T07:50:45.5931502Z ------------------------------------------
2019-07-20T07:50:45.5931538Z 
2019-07-20T07:50:45.5932394Z ------------------------------------------
2019-07-20T07:50:45.5932465Z stderr:
---
2019-07-20T07:50:45.5941267Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-20T07:50:45.5941338Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T07:50:45.5941396Z 
2019-07-20T07:50:45.5941424Z 
2019-07-20T07:50:45.5943068Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-20T07:50:45.5943324Z 
2019-07-20T07:50:45.5943347Z 
2019-07-20T07:50:45.5983438Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-20T07:50:45.5983529Z Build completed unsuccessfully in 0:55:31
2019-07-20T07:50:45.5983529Z Build completed unsuccessfully in 0:55:31
2019-07-20T07:50:46.7261482Z ##[error]Bash exited with code '1'.
2019-07-20T07:50:46.7294415Z ##[section]Starting: Checkout
2019-07-20T07:50:46.7295817Z ==============================================================================
2019-07-20T07:50:46.7295861Z Task         : Get sources
2019-07-20T07:50:46.7295918Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-09-21T23:50:54.7123746Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T23:50:55.2719885Z ##[command]git config gc.auto 0
2019-09-21T23:50:55.2725831Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T23:50:55.2729857Z ##[command]git config --get-all http.proxy
2019-09-21T23:50:55.2734637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64675/merge:refs/remotes/pull/64675/merge
---
2019-09-22T00:53:42.1482236Z .................................................................................................... 1500/9030
2019-09-22T00:53:48.2441896Z .................................................................................................... 1600/9030
2019-09-22T00:54:01.1476637Z .....................................................................i...............i.............. 1700/9030
2019-09-22T00:54:07.9136126Z .................................................................................................... 1800/9030
2019-09-22T00:54:23.5499459Z ............................................................iiiii................................... 1900/9030
2019-09-22T00:54:35.9730549Z .................................................................................................... 2100/9030
2019-09-22T00:54:38.5213171Z .................................................................................................... 2200/9030
2019-09-22T00:54:41.8896242Z .................................................................................................... 2300/9030
2019-09-22T00:54:50.5880525Z .................................................................................................... 2400/9030
---
2019-09-22T00:57:52.1253354Z ................................................i...............i................................... 4700/9030
2019-09-22T00:58:01.7490839Z .................................................................................................... 4800/9030
2019-09-22T00:58:09.6729845Z .................................................................................................... 4900/9030
2019-09-22T00:58:19.3701765Z .................................................................................................... 5000/9030
2019-09-22T00:58:27.3239073Z ..................................ii.ii............................................................. 5100/9030
2019-09-22T00:58:36.8313417Z .................................................................................................... 5300/9030
2019-09-22T00:58:47.5453043Z ..................................................................................................i. 5400/9030
2019-09-22T00:58:55.8080937Z .................................................................................................... 5500/9030
2019-09-22T00:59:00.8222115Z .................................................................................................... 5600/9030
2019-09-22T00:59:00.8222115Z .................................................................................................... 5600/9030
2019-09-22T00:59:11.8436018Z .............................................................................................ii...i. 5700/9030
2019-09-22T00:59:26.0612004Z .ii...........i..................................................................................... 5800/9030
2019-09-22T00:59:47.4629732Z .................................................................................................... 6000/9030
2019-09-22T00:59:56.6320512Z ...............................................................................................i..ii 6100/9030
2019-09-22T01:00:12.0400201Z .................................................................................................... 6200/9030
2019-09-22T01:00:25.8624150Z .................................................................................................... 6300/9030
---
2019-09-22T01:05:14.8415306Z  finished in 5.544
2019-09-22T01:05:14.8615809Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:05:15.0354358Z 
2019-09-22T01:05:15.0355868Z running 150 tests
2019-09-22T01:05:18.3794515Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-22T01:05:20.4137107Z ..iiii..............i.........iii.i.......ii......
2019-09-22T01:05:20.4191916Z 
2019-09-22T01:05:20.4191969Z  finished in 5.551
2019-09-22T01:05:20.4333062Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:05:20.6013251Z 
---
2019-09-22T01:05:22.7628950Z  finished in 2.329
2019-09-22T01:05:22.7813791Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:05:22.9441885Z 
2019-09-22T01:05:22.9442750Z running 9 tests
2019-09-22T01:05:22.9443996Z iiiiiiiii
2019-09-22T01:05:22.9445380Z 
2019-09-22T01:05:22.9448383Z  finished in 0.162
2019-09-22T01:05:22.9633676Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:05:23.1342482Z 
---
2019-09-22T01:05:41.7929246Z  finished in 18.829
2019-09-22T01:05:41.8164429Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:05:41.9936256Z 
2019-09-22T01:05:41.9937280Z running 123 tests
2019-09-22T01:06:06.8345101Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-22T01:06:11.5233234Z i.i.i......iii.i.....ii
2019-09-22T01:06:11.5233793Z 
2019-09-22T01:06:11.5237185Z  finished in 29.706
2019-09-22T01:06:11.5242891Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:06:11.5243242Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-09-22T01:06:11.5243242Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-09-22T01:06:11.5461495Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-22T01:06:11.7077539Z 
2019-09-22T01:06:11.7077903Z running 69 tests
2019-09-22T01:07:14.2563088Z ........................F.F.......FFFF..FFFFFFFF..F..F.FF.FFF..F.....
2019-09-22T01:07:14.2605805Z 
2019-09-22T01:07:14.2606390Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2019-09-22T01:07:14.2606439Z diff of stderr:
2019-09-22T01:07:14.2606467Z 
2019-09-22T01:07:14.2606467Z 
2019-09-22T01:07:14.2607141Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2607374Z +   --> $DIR/issue-15778-fail.rs:6:1
2019-09-22T01:07:14.2607433Z +    |
2019-09-22T01:07:14.2607471Z + LL | #![plugin(lint_for_crate)]
2019-09-22T01:07:14.2607567Z +    |
2019-09-22T01:07:14.2607605Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2607641Z + 
2019-09-22T01:07:14.2607641Z + 
2019-09-22T01:07:14.2607679Z 1 error: crate is not marked with #![crate_okay]
2019-09-22T01:07:14.2608131Z 3    |
2019-09-22T01:07:14.2608154Z 
2019-09-22T01:07:14.2608176Z 
2019-09-22T01:07:14.2608242Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2608242Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2608533Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/issue-15778-fail.stderr
2019-09-22T01:07:14.2608761Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2609326Z To only update this specific test, also pass `--test-args issue-15778-fail.rs`
2019-09-22T01:07:14.2609407Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2609468Z status: exit code: 1
2019-09-22T01:07:14.2609468Z status: exit code: 1
2019-09-22T01:07:14.2610250Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2610613Z ------------------------------------------
2019-09-22T01:07:14.2610648Z 
2019-09-22T01:07:14.2610892Z ------------------------------------------
2019-09-22T01:07:14.2610940Z stderr:
2019-09-22T01:07:14.2610940Z stderr:
2019-09-22T01:07:14.2611161Z ------------------------------------------
2019-09-22T01:07:14.2611558Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2611876Z    |
2019-09-22T01:07:14.2611876Z    |
2019-09-22T01:07:14.2611937Z LL | #![plugin(lint_for_crate)]
2019-09-22T01:07:14.2612038Z    |
2019-09-22T01:07:14.2612107Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2612137Z 
2019-09-22T01:07:14.2612137Z 
2019-09-22T01:07:14.2612181Z error: crate is not marked with #![crate_okay]
2019-09-22T01:07:14.2612790Z    |
2019-09-22T01:07:14.2612790Z    |
2019-09-22T01:07:14.2612828Z LL | / #![feature(plugin)] //~ ERROR crate is not marked with #![crate_okay]
2019-09-22T01:07:14.2612868Z LL | | #![plugin(lint_for_crate)]
2019-09-22T01:07:14.2612952Z LL | | pub fn main() { }
2019-09-22T01:07:14.2612986Z    | |_________________^
2019-09-22T01:07:14.2613036Z    |
2019-09-22T01:07:14.2613036Z    |
2019-09-22T01:07:14.2613232Z    = note: requested on the command line with `-D crate-not-okay`
2019-09-22T01:07:14.2613294Z error: aborting due to previous error
2019-09-22T01:07:14.2613331Z 
2019-09-22T01:07:14.2613352Z 
2019-09-22T01:07:14.2613539Z ------------------------------------------
2019-09-22T01:07:14.2613539Z ------------------------------------------
2019-09-22T01:07:14.2613566Z 
2019-09-22T01:07:14.2613586Z 
2019-09-22T01:07:14.2613880Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-09-22T01:07:14.2613930Z normalized stderr:
2019-09-22T01:07:14.2614270Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2614502Z    |
2019-09-22T01:07:14.2614502Z    |
2019-09-22T01:07:14.2614536Z LL | #![plugin(lint_for_crate_rpass)]
2019-09-22T01:07:14.2614625Z    |
2019-09-22T01:07:14.2614661Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2614702Z 
2019-09-22T01:07:14.2614722Z 
2019-09-22T01:07:14.2614722Z 
2019-09-22T01:07:14.2614742Z 
2019-09-22T01:07:14.2614762Z 
2019-09-22T01:07:14.2614798Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2615067Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/issue-15778-pass.stderr
2019-09-22T01:07:14.2615382Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2615596Z To only update this specific test, also pass `--test-args issue-15778-pass.rs`
2019-09-22T01:07:14.2615677Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2615712Z status: exit code: 0
2019-09-22T01:07:14.2615712Z status: exit code: 0
2019-09-22T01:07:14.2616491Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-09-22T01:07:14.2617002Z ------------------------------------------
2019-09-22T01:07:14.2617039Z 
2019-09-22T01:07:14.2617227Z ------------------------------------------
2019-09-22T01:07:14.2617266Z stderr:
2019-09-22T01:07:14.2617266Z stderr:
2019-09-22T01:07:14.2617464Z ------------------------------------------
2019-09-22T01:07:14.2617773Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2618054Z    |
2019-09-22T01:07:14.2618054Z    |
2019-09-22T01:07:14.2618091Z LL | #![plugin(lint_for_crate_rpass)]
2019-09-22T01:07:14.2620463Z    |
2019-09-22T01:07:14.2620510Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2620544Z 
2019-09-22T01:07:14.2620588Z 
2019-09-22T01:07:14.2620588Z 
2019-09-22T01:07:14.2621092Z ------------------------------------------
2019-09-22T01:07:14.2621148Z 
2019-09-22T01:07:14.2621174Z 
2019-09-22T01:07:14.2621450Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-09-22T01:07:14.2621504Z normalized stderr:
2019-09-22T01:07:14.2621932Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2622201Z   --> $DIR/issue-40001.rs:6:1
2019-09-22T01:07:14.2622250Z    |
2019-09-22T01:07:14.2622296Z LL | #![plugin(issue_40001_plugin)]
2019-09-22T01:07:14.2622409Z    |
2019-09-22T01:07:14.2622455Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2622485Z 
2019-09-22T01:07:14.2622527Z 
2019-09-22T01:07:14.2622527Z 
2019-09-22T01:07:14.2622553Z 
2019-09-22T01:07:14.2622579Z 
2019-09-22T01:07:14.2622625Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2622958Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/issue-40001.stderr
2019-09-22T01:07:14.2623359Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2623768Z To only update this specific test, also pass `--test-args issue-40001.rs`
2019-09-22T01:07:14.2623875Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2623913Z status: exit code: 0
2019-09-22T01:07:14.2623913Z status: exit code: 0
2019-09-22T01:07:14.2624569Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-09-22T01:07:14.2624959Z ------------------------------------------
2019-09-22T01:07:14.2625009Z 
2019-09-22T01:07:14.2625359Z ------------------------------------------
2019-09-22T01:07:14.2625406Z stderr:
2019-09-22T01:07:14.2625406Z stderr:
2019-09-22T01:07:14.2625779Z ------------------------------------------
2019-09-22T01:07:14.2626503Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2626902Z   --> /checkout/src/test/ui-fulldeps/issue-40001.rs:6:1
2019-09-22T01:07:14.2626946Z    |
2019-09-22T01:07:14.2627189Z LL | #![plugin(issue_40001_plugin)]
2019-09-22T01:07:14.2627286Z    |
2019-09-22T01:07:14.2627324Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2627349Z 
2019-09-22T01:07:14.2627371Z 
2019-09-22T01:07:14.2627371Z 
2019-09-22T01:07:14.2627579Z ------------------------------------------
2019-09-22T01:07:14.2627784Z 
2019-09-22T01:07:14.2627807Z 
2019-09-22T01:07:14.2628544Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-09-22T01:07:14.2628630Z diff of stderr:
2019-09-22T01:07:14.2628657Z 
2019-09-22T01:07:14.2629432Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2635671Z +   --> $DIR/lint-group-plugin-deny-cmdline.rs:6:1
2019-09-22T01:07:14.2635730Z +    |
2019-09-22T01:07:14.2635768Z + LL | #![plugin(lint_group_plugin_test)]
2019-09-22T01:07:14.2635870Z +    |
2019-09-22T01:07:14.2635907Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2635959Z + 
2019-09-22T01:07:14.2635959Z + 
2019-09-22T01:07:14.2636153Z 1 error: item is named 'lintme'
2019-09-22T01:07:14.2636402Z 3    |
2019-09-22T01:07:14.2636427Z 
2019-09-22T01:07:14.2636448Z 
2019-09-22T01:07:14.2636485Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2636485Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2637005Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/lint-group-plugin-deny-cmdline.stderr
2019-09-22T01:07:14.2637232Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2637469Z To only update this specific test, also pass `--test-args lint-group-plugin-deny-cmdline.rs`
2019-09-22T01:07:14.2637553Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2637591Z status: exit code: 1
2019-09-22T01:07:14.2637591Z status: exit code: 1
2019-09-22T01:07:14.2638616Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2638977Z ------------------------------------------
2019-09-22T01:07:14.2639198Z 
2019-09-22T01:07:14.2639983Z ------------------------------------------
2019-09-22T01:07:14.2640038Z stderr:
2019-09-22T01:07:14.2640038Z stderr:
2019-09-22T01:07:14.2640322Z ------------------------------------------
2019-09-22T01:07:14.2641672Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2642076Z    |
2019-09-22T01:07:14.2642076Z    |
2019-09-22T01:07:14.2642121Z LL | #![plugin(lint_group_plugin_test)]
2019-09-22T01:07:14.2642410Z    |
2019-09-22T01:07:14.2642456Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2642488Z 
2019-09-22T01:07:14.2642488Z 
2019-09-22T01:07:14.2642902Z error: item is named 'lintme'
2019-09-22T01:07:14.2643157Z    |
2019-09-22T01:07:14.2643157Z    |
2019-09-22T01:07:14.2643368Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2643442Z    |
2019-09-22T01:07:14.2643646Z    = note: `-D test-lint` implied by `-D lint-me`
2019-09-22T01:07:14.2643674Z 
2019-09-22T01:07:14.2643674Z 
2019-09-22T01:07:14.2643851Z error: item is named 'pleaselintme'
2019-09-22T01:07:14.2644365Z    |
2019-09-22T01:07:14.2644365Z    |
2019-09-22T01:07:14.2647128Z LL | fn pleaselintme() { } //~ ERROR item is named 'pleaselintme'
2019-09-22T01:07:14.2647271Z    |
2019-09-22T01:07:14.2647271Z    |
2019-09-22T01:07:14.2648607Z    = note: `-D please-lint` implied by `-D lint-me`
2019-09-22T01:07:14.2648716Z error: aborting due to 2 previous errors
2019-09-22T01:07:14.2648772Z 
2019-09-22T01:07:14.2649638Z 
2019-09-22T01:07:14.2649994Z ------------------------------------------
2019-09-22T01:07:14.2649994Z ------------------------------------------
2019-09-22T01:07:14.2650028Z 
2019-09-22T01:07:14.2650054Z 
2019-09-22T01:07:14.2650316Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-09-22T01:07:14.2650367Z diff of stderr:
2019-09-22T01:07:14.2650396Z 
2019-09-22T01:07:14.2659351Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2659725Z +   --> $DIR/lint-group-plugin.rs:6:1
2019-09-22T01:07:14.2659779Z +    |
2019-09-22T01:07:14.2659848Z + LL | #![plugin(lint_group_plugin_test)]
2019-09-22T01:07:14.2659946Z +    |
2019-09-22T01:07:14.2660008Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2660075Z + 
2019-09-22T01:07:14.2660075Z + 
2019-09-22T01:07:14.2660304Z 1 warning: item is named 'lintme'
2019-09-22T01:07:14.2660609Z 3    |
2019-09-22T01:07:14.2660639Z 
2019-09-22T01:07:14.2660664Z 
2019-09-22T01:07:14.2660711Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2660711Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2661047Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/lint-group-plugin.stderr
2019-09-22T01:07:14.2661302Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2661569Z To only update this specific test, also pass `--test-args lint-group-plugin.rs`
2019-09-22T01:07:14.2661668Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2661714Z status: exit code: 0
2019-09-22T01:07:14.2661714Z status: exit code: 0
2019-09-22T01:07:14.2662814Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-09-22T01:07:14.2663164Z ------------------------------------------
2019-09-22T01:07:14.2663194Z 
2019-09-22T01:07:14.2663381Z ------------------------------------------
2019-09-22T01:07:14.2663421Z stderr:
2019-09-22T01:07:14.2663421Z stderr:
2019-09-22T01:07:14.2663622Z ------------------------------------------
2019-09-22T01:07:14.2663947Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2664350Z    |
2019-09-22T01:07:14.2664350Z    |
2019-09-22T01:07:14.2664389Z LL | #![plugin(lint_group_plugin_test)]
2019-09-22T01:07:14.2664495Z    |
2019-09-22T01:07:14.2664533Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2664574Z 
2019-09-22T01:07:14.2664574Z 
2019-09-22T01:07:14.2664991Z warning: item is named 'lintme'
2019-09-22T01:07:14.2665261Z    |
2019-09-22T01:07:14.2665261Z    |
2019-09-22T01:07:14.2665482Z LL | fn lintme() { } //~ WARNING item is named 'lintme'
2019-09-22T01:07:14.2665560Z    |
2019-09-22T01:07:14.2665615Z    = note: `#[warn(test_lint)]` on by default
2019-09-22T01:07:14.2665642Z 
2019-09-22T01:07:14.2665642Z 
2019-09-22T01:07:14.2665828Z warning: item is named 'pleaselintme'
2019-09-22T01:07:14.2666095Z    |
2019-09-22T01:07:14.2666095Z    |
2019-09-22T01:07:14.2666305Z LL | fn pleaselintme() { } //~ WARNING item is named 'pleaselintme'
2019-09-22T01:07:14.2666410Z    |
2019-09-22T01:07:14.2666410Z    |
2019-09-22T01:07:14.2666455Z    = note: `#[warn(please_lint)]` on by default
2019-09-22T01:07:14.2666504Z 
2019-09-22T01:07:14.2666722Z ------------------------------------------
2019-09-22T01:07:14.2666751Z 
2019-09-22T01:07:14.2666773Z 
2019-09-22T01:07:14.2666773Z 
2019-09-22T01:07:14.2666979Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-09-22T01:07:14.2667038Z diff of stderr:
2019-09-22T01:07:14.2667062Z 
2019-09-22T01:07:14.2667447Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2667680Z +   --> $DIR/lint-plugin-cmdline-allow.rs:8:1
2019-09-22T01:07:14.2667721Z +    |
2019-09-22T01:07:14.2667759Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2667863Z +    |
2019-09-22T01:07:14.2667903Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2667939Z + 
2019-09-22T01:07:14.2667939Z + 
2019-09-22T01:07:14.2668000Z 1 warning: function is never used: `lintme`
2019-09-22T01:07:14.2668249Z 3    |
2019-09-22T01:07:14.2668288Z 
2019-09-22T01:07:14.2668310Z 
2019-09-22T01:07:14.2668349Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2668349Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2668714Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/lint-plugin-cmdline-allow.stderr
2019-09-22T01:07:14.2669377Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2669660Z To only update this specific test, also pass `--test-args lint-plugin-cmdline-allow.rs`
2019-09-22T01:07:14.2669759Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2669803Z status: exit code: 0
2019-09-22T01:07:14.2669803Z status: exit code: 0
2019-09-22T01:07:14.2670686Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-09-22T01:07:14.2671077Z ------------------------------------------
2019-09-22T01:07:14.2671129Z 
2019-09-22T01:07:14.2671353Z ------------------------------------------
2019-09-22T01:07:14.2671399Z stderr:
2019-09-22T01:07:14.2671399Z stderr:
2019-09-22T01:07:14.2671632Z ------------------------------------------
2019-09-22T01:07:14.2672011Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2672636Z    |
2019-09-22T01:07:14.2672636Z    |
2019-09-22T01:07:14.2672674Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2672771Z    |
2019-09-22T01:07:14.2672810Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2672835Z 
2019-09-22T01:07:14.2672835Z 
2019-09-22T01:07:14.2672888Z warning: function is never used: `lintme`
2019-09-22T01:07:14.2673157Z    |
2019-09-22T01:07:14.2673157Z    |
2019-09-22T01:07:14.2673193Z LL | fn lintme() { }
2019-09-22T01:07:14.2673280Z    |
2019-09-22T01:07:14.2673316Z note: lint level defined here
2019-09-22T01:07:14.2673551Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:7:9
2019-09-22T01:07:14.2673592Z    |
---
2019-09-22T01:07:14.2674049Z 
2019-09-22T01:07:14.2674250Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-09-22T01:07:14.2674465Z diff of stderr:
2019-09-22T01:07:14.2674506Z 
2019-09-22T01:07:14.2674819Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2675042Z +   --> $DIR/lint-plugin-deny-attr.rs:5:1
2019-09-22T01:07:14.2675081Z +    |
2019-09-22T01:07:14.2675117Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2675214Z +    |
2019-09-22T01:07:14.2675252Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2675295Z + 
2019-09-22T01:07:14.2675295Z + 
2019-09-22T01:07:14.2675492Z 1 error: item is named 'lintme'
2019-09-22T01:07:14.2675724Z 3    |
2019-09-22T01:07:14.2675747Z 
2019-09-22T01:07:14.2675784Z 
2019-09-22T01:07:14.2675821Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2675821Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2676094Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/lint-plugin-deny-attr.stderr
2019-09-22T01:07:14.2676324Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2676554Z To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`
2019-09-22T01:07:14.2676621Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2676673Z status: exit code: 1
2019-09-22T01:07:14.2676673Z status: exit code: 1
2019-09-22T01:07:14.2677378Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2677700Z ------------------------------------------
2019-09-22T01:07:14.2677729Z 
2019-09-22T01:07:14.2677931Z ------------------------------------------
2019-09-22T01:07:14.2677970Z stderr:
2019-09-22T01:07:14.2677970Z stderr:
2019-09-22T01:07:14.2678151Z ------------------------------------------
2019-09-22T01:07:14.2678477Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2678843Z    |
2019-09-22T01:07:14.2678843Z    |
2019-09-22T01:07:14.2678880Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2679404Z    |
2019-09-22T01:07:14.2679449Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2679480Z 
2019-09-22T01:07:14.2679480Z 
2019-09-22T01:07:14.2679963Z error: item is named 'lintme'
2019-09-22T01:07:14.2680325Z    |
2019-09-22T01:07:14.2680325Z    |
2019-09-22T01:07:14.2680556Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2680663Z    |
2019-09-22T01:07:14.2680704Z note: lint level defined here
2019-09-22T01:07:14.2680949Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:6:9
2019-09-22T01:07:14.2681017Z    |
---
2019-09-22T01:07:14.2681564Z 
2019-09-22T01:07:14.2681798Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-09-22T01:07:14.2681846Z diff of stderr:
2019-09-22T01:07:14.2681889Z 
2019-09-22T01:07:14.2682262Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2682508Z +   --> $DIR/lint-plugin-deny-cmdline.rs:6:1
2019-09-22T01:07:14.2682573Z +    |
2019-09-22T01:07:14.2682617Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2682728Z +    |
2019-09-22T01:07:14.2682773Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2682824Z + 
2019-09-22T01:07:14.2682824Z + 
2019-09-22T01:07:14.2683054Z 1 error: item is named 'lintme'
2019-09-22T01:07:14.2683630Z 3    |
2019-09-22T01:07:14.2683655Z 
2019-09-22T01:07:14.2683693Z 
2019-09-22T01:07:14.2683734Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2683734Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2684025Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/lint-plugin-deny-cmdline.stderr
2019-09-22T01:07:14.2684429Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2684667Z To only update this specific test, also pass `--test-args lint-plugin-deny-cmdline.rs`
2019-09-22T01:07:14.2684735Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2684789Z status: exit code: 1
2019-09-22T01:07:14.2684789Z status: exit code: 1
2019-09-22T01:07:14.2685714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2686058Z ------------------------------------------
2019-09-22T01:07:14.2686087Z 
2019-09-22T01:07:14.2686288Z ------------------------------------------
2019-09-22T01:07:14.2686326Z stderr:
2019-09-22T01:07:14.2686326Z stderr:
2019-09-22T01:07:14.2686506Z ------------------------------------------
2019-09-22T01:07:14.2686833Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2687205Z    |
2019-09-22T01:07:14.2687205Z    |
2019-09-22T01:07:14.2687242Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2687342Z    |
2019-09-22T01:07:14.2687380Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2687405Z 
2019-09-22T01:07:14.2687405Z 
2019-09-22T01:07:14.2687605Z error: item is named 'lintme'
2019-09-22T01:07:14.2687874Z    |
2019-09-22T01:07:14.2687874Z    |
2019-09-22T01:07:14.2688066Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2688155Z    |
2019-09-22T01:07:14.2688529Z    = note: requested on the command line with `-D test-lint`
2019-09-22T01:07:14.2688558Z 
2019-09-22T01:07:14.2688612Z error: aborting due to previous error
---
2019-09-22T01:07:14.2689293Z 
2019-09-22T01:07:14.2689582Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2019-09-22T01:07:14.2689632Z diff of stderr:
2019-09-22T01:07:14.2689660Z 
2019-09-22T01:07:14.2689718Z 7 LL | #[allow(test_lint)]
2019-09-22T01:07:14.2689764Z 8    |         ^^^^^^^^^ overruled by previous forbid
2019-09-22T01:07:14.2689806Z 9 
2019-09-22T01:07:14.2690201Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2690448Z +   --> $DIR/lint-plugin-forbid-attrs.rs:5:1
2019-09-22T01:07:14.2690511Z +    |
2019-09-22T01:07:14.2690555Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2690648Z +    |
2019-09-22T01:07:14.2690710Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2690753Z + 
2019-09-22T01:07:14.2690753Z + 
2019-09-22T01:07:14.2690966Z 10 error: item is named 'lintme'
2019-09-22T01:07:14.2691265Z 12    |
2019-09-22T01:07:14.2691300Z 
2019-09-22T01:07:14.2691326Z 
2019-09-22T01:07:14.2691386Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2691386Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2691708Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2019-09-22T01:07:14.2691962Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2692249Z To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
2019-09-22T01:07:14.2692330Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2692552Z status: exit code: 1
2019-09-22T01:07:14.2692552Z status: exit code: 1
2019-09-22T01:07:14.2693283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2693607Z ------------------------------------------
2019-09-22T01:07:14.2693635Z 
2019-09-22T01:07:14.2694016Z ------------------------------------------
2019-09-22T01:07:14.2694056Z stderr:
2019-09-22T01:07:14.2694056Z stderr:
2019-09-22T01:07:14.2694245Z ------------------------------------------
2019-09-22T01:07:14.2694291Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2019-09-22T01:07:14.2694575Z    |
2019-09-22T01:07:14.2694575Z    |
2019-09-22T01:07:14.2694611Z LL | #![forbid(test_lint)]
2019-09-22T01:07:14.2695092Z    |           --------- `forbid` level set here
2019-09-22T01:07:14.2695132Z ...
2019-09-22T01:07:14.2695176Z LL | #[allow(test_lint)]
2019-09-22T01:07:14.2695401Z    |         ^^^^^^^^^ overruled by previous forbid
2019-09-22T01:07:14.2695428Z 
2019-09-22T01:07:14.2695753Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2696235Z    |
2019-09-22T01:07:14.2696235Z    |
2019-09-22T01:07:14.2696448Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2696552Z    |
2019-09-22T01:07:14.2696592Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2696636Z 
2019-09-22T01:07:14.2696636Z 
2019-09-22T01:07:14.2697021Z error: item is named 'lintme'
2019-09-22T01:07:14.2697464Z    |
2019-09-22T01:07:14.2697464Z    |
2019-09-22T01:07:14.2697870Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2697964Z    |
2019-09-22T01:07:14.2698020Z note: lint level defined here
2019-09-22T01:07:14.2701285Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:6:11
2019-09-22T01:07:14.2701363Z    |
2019-09-22T01:07:14.2701363Z    |
2019-09-22T01:07:14.2701429Z LL | #![forbid(test_lint)]
2019-09-22T01:07:14.2701503Z 
2019-09-22T01:07:14.2701547Z error: aborting due to 2 previous errors
2019-09-22T01:07:14.2701594Z 
2019-09-22T01:07:14.2701866Z For more information about this error, try `rustc --explain E0453`.
---
2019-09-22T01:07:14.2702594Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2019-09-22T01:07:14.2702637Z diff of stderr:
2019-09-22T01:07:14.2702677Z 
2019-09-22T01:07:14.2702728Z 6    |
2019-09-22T01:07:14.2702766Z 7    = note: `forbid` lint level was set on command line
2019-09-22T01:07:14.2702809Z 8 
2019-09-22T01:07:14.2703174Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2703391Z +   --> $DIR/lint-plugin-forbid-cmdline.rs:6:1
2019-09-22T01:07:14.2703446Z +    |
2019-09-22T01:07:14.2703483Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2703561Z +    |
2019-09-22T01:07:14.2703616Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2703651Z + 
2019-09-22T01:07:14.2703651Z + 
2019-09-22T01:07:14.2703833Z 9 error: item is named 'lintme'
2019-09-22T01:07:14.2704084Z 11    |
2019-09-22T01:07:14.2704106Z 
2019-09-22T01:07:14.2704128Z 
2019-09-22T01:07:14.2704189Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2704189Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2704813Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/lint-plugin-forbid-cmdline.stderr
2019-09-22T01:07:14.2705089Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2705343Z To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`
2019-09-22T01:07:14.2705414Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2705452Z status: exit code: 1
2019-09-22T01:07:14.2705452Z status: exit code: 1
2019-09-22T01:07:14.2706154Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2706566Z ------------------------------------------
2019-09-22T01:07:14.2706597Z 
2019-09-22T01:07:14.2706790Z ------------------------------------------
2019-09-22T01:07:14.2706846Z stderr:
2019-09-22T01:07:14.2706846Z stderr:
2019-09-22T01:07:14.2707035Z ------------------------------------------
2019-09-22T01:07:14.2707081Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2019-09-22T01:07:14.2707365Z    |
2019-09-22T01:07:14.2707365Z    |
2019-09-22T01:07:14.2707408Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2019-09-22T01:07:14.2707471Z    |         ^^^^^^^^^ overruled by previous forbid
2019-09-22T01:07:14.2707510Z    |
2019-09-22T01:07:14.2707556Z    = note: `forbid` lint level was set on command line
2019-09-22T01:07:14.2707600Z 
2019-09-22T01:07:14.2707937Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2708240Z    |
2019-09-22T01:07:14.2708240Z    |
2019-09-22T01:07:14.2708278Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2708376Z    |
2019-09-22T01:07:14.2708414Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2708440Z 
2019-09-22T01:07:14.2708440Z 
2019-09-22T01:07:14.2708645Z error: item is named 'lintme'
2019-09-22T01:07:14.2708905Z    |
2019-09-22T01:07:14.2708905Z    |
2019-09-22T01:07:14.2709106Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2709209Z    |
2019-09-22T01:07:14.2709417Z    = note: requested on the command line with `-F test-lint`
2019-09-22T01:07:14.2709462Z 
2019-09-22T01:07:14.2709505Z error: aborting due to 2 previous errors
---
2019-09-22T01:07:14.2711179Z 
2019-09-22T01:07:14.2711426Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2019-09-22T01:07:14.2711475Z diff of stderr:
2019-09-22T01:07:14.2711502Z 
2019-09-22T01:07:14.2711895Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2712129Z +   --> $DIR/lint-plugin.rs:5:1
2019-09-22T01:07:14.2712175Z +    |
2019-09-22T01:07:14.2712218Z + LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2712340Z +    |
2019-09-22T01:07:14.2712512Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2712591Z + 
2019-09-22T01:07:14.2712591Z + 
2019-09-22T01:07:14.2712838Z 1 warning: item is named 'lintme'
2019-09-22T01:07:14.2713114Z 3    |
2019-09-22T01:07:14.2713141Z 
2019-09-22T01:07:14.2713167Z 
2019-09-22T01:07:14.2713211Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2713211Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2713741Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/lint-plugin.stderr
2019-09-22T01:07:14.2714149Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2714536Z To only update this specific test, also pass `--test-args lint-plugin.rs`
2019-09-22T01:07:14.2714780Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2714818Z status: exit code: 0
2019-09-22T01:07:14.2714818Z status: exit code: 0
2019-09-22T01:07:14.2715438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2019-09-22T01:07:14.2715989Z ------------------------------------------
2019-09-22T01:07:14.2716035Z 
2019-09-22T01:07:14.2716224Z ------------------------------------------
2019-09-22T01:07:14.2716262Z stderr:
2019-09-22T01:07:14.2716262Z stderr:
2019-09-22T01:07:14.2716459Z ------------------------------------------
2019-09-22T01:07:14.2716772Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2717062Z    |
2019-09-22T01:07:14.2717062Z    |
2019-09-22T01:07:14.2717104Z LL | #![plugin(lint_plugin_test)]
2019-09-22T01:07:14.2717199Z    |
2019-09-22T01:07:14.2717236Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2717261Z 
2019-09-22T01:07:14.2717261Z 
2019-09-22T01:07:14.2717458Z warning: item is named 'lintme'
2019-09-22T01:07:14.2717699Z    |
2019-09-22T01:07:14.2717699Z    |
2019-09-22T01:07:14.2717894Z LL | fn lintme() { } //~ WARNING item is named 'lintme'
2019-09-22T01:07:14.2717984Z    |
2019-09-22T01:07:14.2718021Z    = note: `#[warn(test_lint)]` on by default
2019-09-22T01:07:14.2718063Z 
2019-09-22T01:07:14.2718085Z 
---
2019-09-22T01:07:14.2718609Z 
2019-09-22T01:07:14.2718642Z 2    |
2019-09-22T01:07:14.2718863Z 3    = note: requested on the command line with `-A test_lint`
2019-09-22T01:07:14.2718903Z 4 
2019-09-22T01:07:14.2719819Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2720073Z +   --> $DIR/lint-tool-cmdline-allow.rs:8:1
2019-09-22T01:07:14.2720119Z +    |
2019-09-22T01:07:14.2720179Z + LL | #![plugin(lint_tool_test)]
2019-09-22T01:07:14.2720271Z +    |
2019-09-22T01:07:14.2720315Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2720374Z + 
2019-09-22T01:07:14.2720374Z + 
2019-09-22T01:07:14.2720586Z 5 warning: item is named 'lintme'
2019-09-22T01:07:14.2720885Z 7    |
2019-09-22T01:07:14.2720912Z 
2019-09-22T01:07:14.2720938Z 
2019-09-22T01:07:14.2721094Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2721094Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2721481Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/lint-tool-cmdline-allow.stderr
2019-09-22T01:07:14.2721735Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2722004Z To only update this specific test, also pass `--test-args lint-tool-cmdline-allow.rs`
2019-09-22T01:07:14.2722099Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2722143Z status: exit code: 0
2019-09-22T01:07:14.2722143Z status: exit code: 0
2019-09-22T01:07:14.2723045Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2019-09-22T01:07:14.2723439Z ------------------------------------------
2019-09-22T01:07:14.2723469Z 
2019-09-22T01:07:14.2723656Z ------------------------------------------
2019-09-22T01:07:14.2723695Z stderr:
2019-09-22T01:07:14.2723695Z stderr:
2019-09-22T01:07:14.2723895Z ------------------------------------------
2019-09-22T01:07:14.2723944Z warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
2019-09-22T01:07:14.2724524Z    = note: requested on the command line with `-A test_lint`
2019-09-22T01:07:14.2724568Z 
2019-09-22T01:07:14.2724568Z 
2019-09-22T01:07:14.2724906Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2725231Z    |
2019-09-22T01:07:14.2725231Z    |
2019-09-22T01:07:14.2725268Z LL | #![plugin(lint_tool_test)]
2019-09-22T01:07:14.2725362Z    |
2019-09-22T01:07:14.2725399Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2725441Z 
2019-09-22T01:07:14.2725441Z 
2019-09-22T01:07:14.2725623Z warning: item is named 'lintme'
2019-09-22T01:07:14.2725875Z    |
2019-09-22T01:07:14.2725875Z    |
2019-09-22T01:07:14.2725926Z LL | fn lintme() {}
2019-09-22T01:07:14.2725994Z    |
2019-09-22T01:07:14.2726047Z    = note: `#[warn(clippy::test_lint)]` on by default
2019-09-22T01:07:14.2726071Z 
2019-09-22T01:07:14.2726071Z 
2019-09-22T01:07:14.2726107Z warning: function is never used: `lintme`
2019-09-22T01:07:14.2726383Z    |
2019-09-22T01:07:14.2726383Z    |
2019-09-22T01:07:14.2726417Z LL | fn lintme() {}
2019-09-22T01:07:14.2726509Z    |
2019-09-22T01:07:14.2726544Z note: lint level defined here
2019-09-22T01:07:14.2726755Z   --> /checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs:7:9
2019-09-22T01:07:14.2726811Z    |
---
2019-09-22T01:07:14.2727221Z 
2019-09-22T01:07:14.2727429Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2019-09-22T01:07:14.2727470Z diff of stderr:
2019-09-22T01:07:14.2727492Z 
2019-09-22T01:07:14.2727543Z 32 LL | #![cfg_attr(foo, warn(test_lint))]
2019-09-22T01:07:14.2727594Z 33    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2019-09-22T01:07:14.2727632Z 34 
2019-09-22T01:07:14.2728064Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2728312Z +   --> $DIR/lint-tool-test.rs:6:1
2019-09-22T01:07:14.2728354Z +    |
2019-09-22T01:07:14.2728407Z + LL | #![plugin(lint_tool_test)]
2019-09-22T01:07:14.2728484Z +    |
2019-09-22T01:07:14.2728537Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2728572Z + 
2019-09-22T01:07:14.2728572Z + 
2019-09-22T01:07:14.2728755Z 35 error: item is named 'lintme'
2019-09-22T01:07:14.2729567Z 37    |
2019-09-22T01:07:14.2729594Z 
2019-09-22T01:07:14.2729619Z 
2019-09-22T01:07:14.2729663Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2729663Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2730142Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2019-09-22T01:07:14.2730403Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2730681Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2019-09-22T01:07:14.2730762Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2730807Z status: exit code: 1
2019-09-22T01:07:14.2730807Z status: exit code: 1
2019-09-22T01:07:14.2731564Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2731911Z ------------------------------------------
2019-09-22T01:07:14.2731945Z 
2019-09-22T01:07:14.2732166Z ------------------------------------------
2019-09-22T01:07:14.2732228Z stderr:
2019-09-22T01:07:14.2732228Z stderr:
2019-09-22T01:07:14.2732598Z ------------------------------------------
2019-09-22T01:07:14.2733034Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2019-09-22T01:07:14.2733308Z    |
2019-09-22T01:07:14.2733308Z    |
2019-09-22T01:07:14.2733344Z LL | #![cfg_attr(foo, warn(test_lint))]
2019-09-22T01:07:14.2733402Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2019-09-22T01:07:14.2733478Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2019-09-22T01:07:14.2733503Z 
2019-09-22T01:07:14.2733503Z 
2019-09-22T01:07:14.2733806Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2019-09-22T01:07:14.2734062Z    |
2019-09-22T01:07:14.2734114Z LL | #![deny(clippy_group)]
2019-09-22T01:07:14.2734114Z LL | #![deny(clippy_group)]
2019-09-22T01:07:14.2734154Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2019-09-22T01:07:14.2734179Z 
2019-09-22T01:07:14.2734461Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2019-09-22T01:07:14.2734712Z    |
2019-09-22T01:07:14.2734712Z    |
2019-09-22T01:07:14.2734768Z LL | #[allow(test_group)]
2019-09-22T01:07:14.2734809Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2019-09-22T01:07:14.2734834Z 
2019-09-22T01:07:14.2734890Z warning: unknown lint: `this_lint_does_not_exist`
2019-09-22T01:07:14.2735145Z    |
2019-09-22T01:07:14.2735145Z    |
2019-09-22T01:07:14.2735282Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2019-09-22T01:07:14.2735371Z    |
2019-09-22T01:07:14.2735408Z    = note: `#[warn(unknown_lints)]` on by default
2019-09-22T01:07:14.2735453Z 
2019-09-22T01:07:14.2735453Z 
2019-09-22T01:07:14.2735748Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2019-09-22T01:07:14.2736015Z    |
2019-09-22T01:07:14.2736015Z    |
2019-09-22T01:07:14.2736051Z LL | #![cfg_attr(foo, warn(test_lint))]
2019-09-22T01:07:14.2736094Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2019-09-22T01:07:14.2736120Z 
2019-09-22T01:07:14.2736456Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2736864Z    |
2019-09-22T01:07:14.2736864Z    |
2019-09-22T01:07:14.2736899Z LL | #![plugin(lint_tool_test)]
2019-09-22T01:07:14.2736993Z    |
2019-09-22T01:07:14.2737030Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2737056Z 
2019-09-22T01:07:14.2737056Z 
2019-09-22T01:07:14.2737235Z error: item is named 'lintme'
2019-09-22T01:07:14.2737495Z    |
2019-09-22T01:07:14.2737495Z    |
2019-09-22T01:07:14.2737690Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2019-09-22T01:07:14.2737784Z    |
2019-09-22T01:07:14.2737818Z note: lint level defined here
2019-09-22T01:07:14.2738020Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:11:9
2019-09-22T01:07:14.2738087Z    |
2019-09-22T01:07:14.2738087Z    |
2019-09-22T01:07:14.2738442Z LL | #![deny(clippy_group)]
2019-09-22T01:07:14.2738481Z    |         ^^^^^^^^^^^^
2019-09-22T01:07:14.2738552Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2019-09-22T01:07:14.2738580Z 
2019-09-22T01:07:14.2738799Z error: item is named 'lintmetoo'
2019-09-22T01:07:14.2740053Z    |
2019-09-22T01:07:14.2740053Z    |
2019-09-22T01:07:14.2740354Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2019-09-22T01:07:14.2740467Z    |
2019-09-22T01:07:14.2740508Z note: lint level defined here
2019-09-22T01:07:14.2740747Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:11:9
2019-09-22T01:07:14.2740815Z    |
2019-09-22T01:07:14.2740815Z    |
2019-09-22T01:07:14.2740857Z LL | #![deny(clippy_group)]
2019-09-22T01:07:14.2740899Z    |         ^^^^^^^^^^^^
2019-09-22T01:07:14.2741024Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2019-09-22T01:07:14.2741113Z error: aborting due to 2 previous errors
2019-09-22T01:07:14.2741142Z 
2019-09-22T01:07:14.2741175Z 
2019-09-22T01:07:14.2741427Z ------------------------------------------
2019-09-22T01:07:14.2741427Z ------------------------------------------
2019-09-22T01:07:14.2741460Z 
2019-09-22T01:07:14.2741485Z 
2019-09-22T01:07:14.2741710Z ---- [ui] ui-fulldeps/llvm-pass-plugin.rs stdout ----
2019-09-22T01:07:14.2741779Z normalized stderr:
2019-09-22T01:07:14.2742157Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2742458Z    |
2019-09-22T01:07:14.2742458Z    |
2019-09-22T01:07:14.2742500Z LL | #![plugin(llvm_pass_plugin)]
2019-09-22T01:07:14.2742771Z    |
2019-09-22T01:07:14.2742811Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2742837Z 
2019-09-22T01:07:14.2743027Z 
2019-09-22T01:07:14.2743027Z 
2019-09-22T01:07:14.2743076Z 
2019-09-22T01:07:14.2743097Z 
2019-09-22T01:07:14.2743135Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2743531Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/llvm-pass-plugin/llvm-pass-plugin.stderr
2019-09-22T01:07:14.2744029Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2744261Z To only update this specific test, also pass `--test-args llvm-pass-plugin.rs`
2019-09-22T01:07:14.2744350Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2744388Z status: exit code: 0
2019-09-22T01:07:14.2744388Z status: exit code: 0
2019-09-22T01:07:14.2745017Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/llvm-pass-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/llvm-pass-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/llvm-pass-plugin/auxiliary"
2019-09-22T01:07:14.2745628Z ------------------------------------------
2019-09-22T01:07:14.2745664Z 
2019-09-22T01:07:14.2745886Z ------------------------------------------
2019-09-22T01:07:14.2745926Z stderr:
2019-09-22T01:07:14.2745926Z stderr:
2019-09-22T01:07:14.2746113Z ------------------------------------------
2019-09-22T01:07:14.2746462Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2746756Z    |
2019-09-22T01:07:14.2746756Z    |
2019-09-22T01:07:14.2746793Z LL | #![plugin(llvm_pass_plugin)]
2019-09-22T01:07:14.2746891Z    |
2019-09-22T01:07:14.2746929Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2746964Z 
2019-09-22T01:07:14.2746987Z 
2019-09-22T01:07:14.2746987Z 
2019-09-22T01:07:14.2747209Z ------------------------------------------
2019-09-22T01:07:14.2747239Z 
2019-09-22T01:07:14.2747261Z 
2019-09-22T01:07:14.2747467Z ---- [ui] ui-fulldeps/lto-syntax-extension.rs stdout ----
2019-09-22T01:07:14.2747531Z normalized stderr:
2019-09-22T01:07:14.2747844Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2748110Z    |
2019-09-22T01:07:14.2748110Z    |
2019-09-22T01:07:14.2748149Z LL | #![plugin(lto_syntax_extension_plugin)]
2019-09-22T01:07:14.2748250Z    |
2019-09-22T01:07:14.2748288Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2748313Z 
2019-09-22T01:07:14.2748335Z 
2019-09-22T01:07:14.2748335Z 
2019-09-22T01:07:14.2748357Z 
2019-09-22T01:07:14.2748405Z 
2019-09-22T01:07:14.2748444Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2748729Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/lto-syntax-extension.stderr
2019-09-22T01:07:14.2749375Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2749652Z To only update this specific test, also pass `--test-args lto-syntax-extension.rs`
2019-09-22T01:07:14.2749732Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2749798Z status: exit code: 0
2019-09-22T01:07:14.2749798Z status: exit code: 0
2019-09-22T01:07:14.2750637Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lto-syntax-extension.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary"
2019-09-22T01:07:14.2751017Z ------------------------------------------
2019-09-22T01:07:14.2751051Z 
2019-09-22T01:07:14.2751291Z ------------------------------------------
2019-09-22T01:07:14.2751337Z stderr:
2019-09-22T01:07:14.2751337Z stderr:
2019-09-22T01:07:14.2751551Z ------------------------------------------
2019-09-22T01:07:14.2751944Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2752283Z    |
2019-09-22T01:07:14.2752283Z    |
2019-09-22T01:07:14.2752587Z LL | #![plugin(lto_syntax_extension_plugin)]
2019-09-22T01:07:14.2752687Z    |
2019-09-22T01:07:14.2752725Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2752839Z 
2019-09-22T01:07:14.2752862Z 
2019-09-22T01:07:14.2752862Z 
2019-09-22T01:07:14.2753084Z ------------------------------------------
2019-09-22T01:07:14.2753135Z 
2019-09-22T01:07:14.2753157Z 
2019-09-22T01:07:14.2753362Z ---- [ui] ui-fulldeps/outlive-expansion-phase.rs stdout ----
2019-09-22T01:07:14.2753405Z normalized stderr:
2019-09-22T01:07:14.2753744Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2754020Z    |
2019-09-22T01:07:14.2754020Z    |
2019-09-22T01:07:14.2754058Z LL | #![plugin(outlive_expansion_phase)]
2019-09-22T01:07:14.2754157Z    |
2019-09-22T01:07:14.2754195Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2754220Z 
2019-09-22T01:07:14.2754242Z 
2019-09-22T01:07:14.2754242Z 
2019-09-22T01:07:14.2754264Z 
2019-09-22T01:07:14.2754312Z 
2019-09-22T01:07:14.2754350Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2754642Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/outlive-expansion-phase.stderr
2019-09-22T01:07:14.2754885Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2755117Z To only update this specific test, also pass `--test-args outlive-expansion-phase.rs`
2019-09-22T01:07:14.2755186Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2755243Z status: exit code: 0
2019-09-22T01:07:14.2755243Z status: exit code: 0
2019-09-22T01:07:14.2755891Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/outlive-expansion-phase.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary"
2019-09-22T01:07:14.2756190Z ------------------------------------------
2019-09-22T01:07:14.2756220Z 
2019-09-22T01:07:14.2756431Z ------------------------------------------
2019-09-22T01:07:14.2756471Z stderr:
2019-09-22T01:07:14.2756471Z stderr:
2019-09-22T01:07:14.2756657Z ------------------------------------------
2019-09-22T01:07:14.2756991Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2757289Z    |
2019-09-22T01:07:14.2757289Z    |
2019-09-22T01:07:14.2757327Z LL | #![plugin(outlive_expansion_phase)]
2019-09-22T01:07:14.2757428Z    |
2019-09-22T01:07:14.2757473Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2757498Z 
2019-09-22T01:07:14.2757521Z 
2019-09-22T01:07:14.2757521Z 
2019-09-22T01:07:14.2757812Z ------------------------------------------
2019-09-22T01:07:14.2757870Z 
2019-09-22T01:07:14.2757892Z 
2019-09-22T01:07:14.2758115Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-09-22T01:07:14.2758156Z normalized stderr:
2019-09-22T01:07:14.2758492Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2758697Z   --> $DIR/plugin-args-1.rs:6:1
2019-09-22T01:07:14.2758756Z    |
2019-09-22T01:07:14.2758793Z LL | #![plugin(plugin_args)]
2019-09-22T01:07:14.2759041Z    |
2019-09-22T01:07:14.2759294Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2759324Z 
2019-09-22T01:07:14.2759349Z 
2019-09-22T01:07:14.2759349Z 
2019-09-22T01:07:14.2759375Z 
2019-09-22T01:07:14.2759419Z 
2019-09-22T01:07:14.2759587Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2759970Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/plugin-args-1.stderr
2019-09-22T01:07:14.2760250Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2760510Z To only update this specific test, also pass `--test-args plugin-args-1.rs`
2019-09-22T01:07:14.2760589Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2760653Z status: exit code: 0
2019-09-22T01:07:14.2760653Z status: exit code: 0
2019-09-22T01:07:14.2761359Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-09-22T01:07:14.2761706Z ------------------------------------------
2019-09-22T01:07:14.2761740Z 
2019-09-22T01:07:14.2761979Z ------------------------------------------
2019-09-22T01:07:14.2762024Z stderr:
2019-09-22T01:07:14.2762024Z stderr:
2019-09-22T01:07:14.2762239Z ------------------------------------------
2019-09-22T01:07:14.2762917Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2763147Z   --> /checkout/src/test/ui-fulldeps/plugin-args-1.rs:6:1
2019-09-22T01:07:14.2763189Z    |
2019-09-22T01:07:14.2763245Z LL | #![plugin(plugin_args)]
2019-09-22T01:07:14.2763323Z    |
2019-09-22T01:07:14.2763380Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2763405Z 
2019-09-22T01:07:14.2763435Z 
2019-09-22T01:07:14.2763435Z 
2019-09-22T01:07:14.2763628Z ------------------------------------------
2019-09-22T01:07:14.2763675Z 
2019-09-22T01:07:14.2763698Z 
2019-09-22T01:07:14.2763903Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-09-22T01:07:14.2763945Z normalized stderr:
2019-09-22T01:07:14.2764275Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2764479Z   --> $DIR/plugin-args-2.rs:6:1
2019-09-22T01:07:14.2764519Z    |
2019-09-22T01:07:14.2764577Z LL | #![plugin(plugin_args())]
2019-09-22T01:07:14.2764656Z    |
2019-09-22T01:07:14.2764713Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2764738Z 
2019-09-22T01:07:14.2764760Z 
2019-09-22T01:07:14.2764760Z 
2019-09-22T01:07:14.2764782Z 
2019-09-22T01:07:14.2764803Z 
2019-09-22T01:07:14.2764863Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2765140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/plugin-args-2.stderr
2019-09-22T01:07:14.2765444Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2765728Z To only update this specific test, also pass `--test-args plugin-args-2.rs`
2019-09-22T01:07:14.2765801Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2765858Z status: exit code: 0
2019-09-22T01:07:14.2765858Z status: exit code: 0
2019-09-22T01:07:14.2766474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-09-22T01:07:14.2766864Z ------------------------------------------
2019-09-22T01:07:14.2766901Z 
2019-09-22T01:07:14.2767276Z ------------------------------------------
2019-09-22T01:07:14.2767397Z stderr:
2019-09-22T01:07:14.2767397Z stderr:
2019-09-22T01:07:14.2767592Z ------------------------------------------
2019-09-22T01:07:14.2767939Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2768176Z   --> /checkout/src/test/ui-fulldeps/plugin-args-2.rs:6:1
2019-09-22T01:07:14.2768221Z    |
2019-09-22T01:07:14.2768278Z LL | #![plugin(plugin_args())]
2019-09-22T01:07:14.2768360Z    |
2019-09-22T01:07:14.2768419Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2768446Z 
2019-09-22T01:07:14.2768469Z 
2019-09-22T01:07:14.2768469Z 
2019-09-22T01:07:14.2768669Z ------------------------------------------
2019-09-22T01:07:14.2768706Z 
2019-09-22T01:07:14.2768748Z 
2019-09-22T01:07:14.2769495Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-09-22T01:07:14.2769658Z diff of stderr:
2019-09-22T01:07:14.2769687Z 
2019-09-22T01:07:14.2770103Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2770354Z +   --> $DIR/plugin-attr-register-deny.rs:5:1
2019-09-22T01:07:14.2770421Z +    |
2019-09-22T01:07:14.2770465Z + LL | #![plugin(attr_plugin_test)]
2019-09-22T01:07:14.2770556Z +    |
2019-09-22T01:07:14.2770620Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2770663Z + 
2019-09-22T01:07:14.2770706Z 1 error: unused attribute
2019-09-22T01:07:14.2770706Z 1 error: unused attribute
2019-09-22T01:07:14.2770953Z 2   --> $DIR/plugin-attr-register-deny.rs:14:5
2019-09-22T01:07:14.2770999Z 3    |
2019-09-22T01:07:14.2771025Z 
2019-09-22T01:07:14.2771051Z 
2019-09-22T01:07:14.2771125Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2771460Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/plugin-attr-register-deny.stderr
2019-09-22T01:07:14.2771719Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2772011Z To only update this specific test, also pass `--test-args plugin-attr-register-deny.rs`
2019-09-22T01:07:14.2772091Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2772156Z status: exit code: 1
2019-09-22T01:07:14.2772156Z status: exit code: 1
2019-09-22T01:07:14.2774223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-attr-register-deny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2774675Z ------------------------------------------
2019-09-22T01:07:14.2774708Z 
2019-09-22T01:07:14.2774907Z ------------------------------------------
2019-09-22T01:07:14.2774969Z stderr:
2019-09-22T01:07:14.2774969Z stderr:
2019-09-22T01:07:14.2775160Z ------------------------------------------
2019-09-22T01:07:14.2775525Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2775817Z    |
2019-09-22T01:07:14.2775817Z    |
2019-09-22T01:07:14.2775875Z LL | #![plugin(attr_plugin_test)]
2019-09-22T01:07:14.2775955Z    |
2019-09-22T01:07:14.2776116Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2776144Z 
2019-09-22T01:07:14.2776179Z error: unused attribute
---
2019-09-22T01:07:14.2777302Z    |
2019-09-22T01:07:14.2777359Z LL | #![deny(unused_attributes)]
2019-09-22T01:07:14.2777631Z    |         ^^^^^^^^^^^^^^^^^
2019-09-22T01:07:14.2777657Z 
2019-09-22T01:07:14.2777920Z error: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
2019-09-22T01:07:14.2778599Z    |
2019-09-22T01:07:14.2778647Z LL |     #[bar]
2019-09-22T01:07:14.2778708Z    |     ^^^^^^
2019-09-22T01:07:14.2778734Z 
---
2019-09-22T01:07:14.2780486Z 
2019-09-22T01:07:14.2780527Z 10 LL | pub use mac as reexport;
2019-09-22T01:07:14.2780572Z 11    |         ^^^^^^^^^^^^^^^
2019-09-22T01:07:14.2780632Z 12 
2019-09-22T01:07:14.2781016Z + warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2781299Z +   --> $DIR/plugin-reexport.rs:6:1
2019-09-22T01:07:14.2781348Z +    |
2019-09-22T01:07:14.2781390Z + LL | #![plugin(attr_plugin_test)]
2019-09-22T01:07:14.2781502Z +    |
2019-09-22T01:07:14.2781548Z +    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2781591Z + 
2019-09-22T01:07:14.2781653Z 13 error: aborting due to previous error
2019-09-22T01:07:14.2781653Z 13 error: aborting due to previous error
2019-09-22T01:07:14.2781695Z 14 
2019-09-22T01:07:14.2781949Z 15 For more information about this error, try `rustc --explain E0364`.
2019-09-22T01:07:14.2781984Z 
2019-09-22T01:07:14.2782031Z 
2019-09-22T01:07:14.2782077Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2782384Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/plugin-reexport.stderr
2019-09-22T01:07:14.2782809Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2783160Z To only update this specific test, also pass `--test-args plugin-reexport.rs`
2019-09-22T01:07:14.2783241Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2783301Z status: exit code: 1
2019-09-22T01:07:14.2783301Z status: exit code: 1
2019-09-22T01:07:14.2784157Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-reexport.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-A" "unused"
2019-09-22T01:07:14.2784575Z ------------------------------------------
2019-09-22T01:07:14.2784608Z 
2019-09-22T01:07:14.2784837Z ------------------------------------------
2019-09-22T01:07:14.2784878Z stderr:
2019-09-22T01:07:14.2784878Z stderr:
2019-09-22T01:07:14.2785072Z ------------------------------------------
2019-09-22T01:07:14.2785307Z error[E0364]: `mac` is private, and cannot be re-exported
2019-09-22T01:07:14.2785577Z    |
2019-09-22T01:07:14.2785577Z    |
2019-09-22T01:07:14.2785827Z LL | pub use mac as reexport; //~ ERROR `mac` is private, and cannot be re-exported
2019-09-22T01:07:14.2785914Z    |
2019-09-22T01:07:14.2785914Z    |
2019-09-22T01:07:14.2785973Z note: consider marking `mac` as `pub` in the imported module
2019-09-22T01:07:14.2786232Z    |
2019-09-22T01:07:14.2786232Z    |
2019-09-22T01:07:14.2786462Z LL | pub use mac as reexport; //~ ERROR `mac` is private, and cannot be re-exported
2019-09-22T01:07:14.2786562Z 
2019-09-22T01:07:14.2786562Z 
2019-09-22T01:07:14.2786920Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2787202Z    |
2019-09-22T01:07:14.2787202Z    |
2019-09-22T01:07:14.2787261Z LL | #![plugin(attr_plugin_test)]
2019-09-22T01:07:14.2787344Z    |
2019-09-22T01:07:14.2787402Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2787428Z 
2019-09-22T01:07:14.2787466Z error: aborting due to previous error
---
2019-09-22T01:07:14.2787990Z 
2019-09-22T01:07:14.2788013Z 
2019-09-22T01:07:14.2788247Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-09-22T01:07:14.2788291Z normalized stderr:
2019-09-22T01:07:14.2788620Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2788856Z   --> $DIR/plugin-args-3.rs:6:1
2019-09-22T01:07:14.2788899Z    |
2019-09-22T01:07:14.2788939Z LL | #![plugin(plugin_args(hello(there), how(are="you")))]
2019-09-22T01:07:14.2789410Z    |
2019-09-22T01:07:14.2789454Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2789504Z 
2019-09-22T01:07:14.2789530Z 
2019-09-22T01:07:14.2789530Z 
2019-09-22T01:07:14.2789555Z 
2019-09-22T01:07:14.2789581Z 
2019-09-22T01:07:14.2789626Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2789996Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/plugin-args-3.stderr
2019-09-22T01:07:14.2790259Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2790637Z To only update this specific test, also pass `--test-args plugin-args-3.rs`
2019-09-22T01:07:14.2790750Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2790794Z status: exit code: 0
2019-09-22T01:07:14.2790794Z status: exit code: 0
2019-09-22T01:07:14.2791551Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-09-22T01:07:14.2791882Z ------------------------------------------
2019-09-22T01:07:14.2792013Z 
2019-09-22T01:07:14.2792263Z ------------------------------------------
2019-09-22T01:07:14.2792309Z stderr:
2019-09-22T01:07:14.2792309Z stderr:
2019-09-22T01:07:14.2792772Z ------------------------------------------
2019-09-22T01:07:14.2793113Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2793370Z   --> /checkout/src/test/ui-fulldeps/plugin-args-3.rs:6:1
2019-09-22T01:07:14.2793416Z    |
2019-09-22T01:07:14.2793456Z LL | #![plugin(plugin_args(hello(there), how(are="you")))]
2019-09-22T01:07:14.2793563Z    |
2019-09-22T01:07:14.2793602Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2793628Z 
2019-09-22T01:07:14.2793670Z 
2019-09-22T01:07:14.2793670Z 
2019-09-22T01:07:14.2793871Z ------------------------------------------
2019-09-22T01:07:14.2793900Z 
2019-09-22T01:07:14.2793923Z 
2019-09-22T01:07:14.2794156Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-09-22T01:07:14.2794200Z normalized stderr:
2019-09-22T01:07:14.2794532Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2794770Z   --> $DIR/roman-numerals-macro.rs:6:1
2019-09-22T01:07:14.2794811Z    |
2019-09-22T01:07:14.2794849Z LL | #![plugin(roman_numerals)]
2019-09-22T01:07:14.2794951Z    |
2019-09-22T01:07:14.2794990Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2795016Z 
2019-09-22T01:07:14.2795059Z 
2019-09-22T01:07:14.2795059Z 
2019-09-22T01:07:14.2795082Z 
2019-09-22T01:07:14.2795104Z 
2019-09-22T01:07:14.2795144Z The actual stderr differed from the expected stderr.
2019-09-22T01:07:14.2795605Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/roman-numerals-macro.stderr
2019-09-22T01:07:14.2795837Z To update references, rerun the tests and pass the `--bless` flag
2019-09-22T01:07:14.2796070Z To only update this specific test, also pass `--test-args roman-numerals-macro.rs`
2019-09-22T01:07:14.2796157Z error: 1 errors occurred comparing output.
2019-09-22T01:07:14.2796196Z status: exit code: 0
2019-09-22T01:07:14.2796196Z status: exit code: 0
2019-09-22T01:07:14.2796846Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/roman-numerals-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-09-22T01:07:14.2797137Z ------------------------------------------
2019-09-22T01:07:14.2797173Z 
2019-09-22T01:07:14.2797365Z ------------------------------------------
2019-09-22T01:07:14.2797497Z stderr:
2019-09-22T01:07:14.2797497Z stderr:
2019-09-22T01:07:14.2797748Z ------------------------------------------
2019-09-22T01:07:14.2798070Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated and will be removed in 1.44.0. See ***/issues/29597
2019-09-22T01:07:14.2798367Z    |
2019-09-22T01:07:14.2798367Z    |
2019-09-22T01:07:14.2798403Z LL | #![plugin(roman_numerals)]
2019-09-22T01:07:14.2798500Z    |
2019-09-22T01:07:14.2798538Z    = note: `#[warn(deprecated)]` on by default
2019-09-22T01:07:14.2798564Z 
2019-09-22T01:07:14.2798605Z 
---
2019-09-22T01:07:14.2805167Z test result: FAILED. 47 passed; 22 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-22T01:07:14.2805199Z 
2019-09-22T01:07:14.2805229Z 
2019-09-22T01:07:14.2805251Z 
2019-09-22T01:07:14.2806655Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-22T01:07:14.2806889Z 
2019-09-22T01:07:14.2806914Z 
2019-09-22T01:07:14.2806976Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-22T01:07:14.2807019Z Build completed unsuccessfully in 1:09:28
2019-09-22T01:07:14.2807019Z Build completed unsuccessfully in 1:09:28
2019-09-22T01:07:14.2807175Z == clock drift check ==
2019-09-22T01:07:14.2807216Z   local time: Sun Sep 22 01:07:14 UTC 2019
2019-09-22T01:07:14.3513725Z   network time: Sun, 22 Sep 2019 01:07:14 GMT
2019-09-22T01:07:14.3517165Z == end clock drift check ==
2019-09-22T01:07:15.2810437Z ##[error]Bash exited with code '1'.
2019-09-22T01:07:15.2864406Z ##[section]Starting: Checkout
2019-09-22T01:07:15.2866118Z ==============================================================================
2019-09-22T01:07:15.2866169Z Task         : Get sources
2019-09-22T01:07:15.2866227Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

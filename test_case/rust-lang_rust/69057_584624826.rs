plain
2020-02-11T11:51:42.8918518Z ========================== Starting Command Output ===========================
2020-02-11T11:51:42.8920212Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8fb66146-8bd9-4791-b487-0b1b677403db.sh
2020-02-11T11:51:42.8920247Z 
2020-02-11T11:51:42.8923294Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T11:51:42.8929445Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69057/merge to s
2020-02-11T11:51:42.8931140Z Task         : Get sources
2020-02-11T11:51:42.8931223Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T11:51:42.8931257Z Version      : 1.0.0
2020-02-11T11:51:42.8931289Z Author       : Microsoft
---
2020-02-11T11:51:43.9382434Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T11:51:43.9396585Z ##[command]git config gc.auto 0
2020-02-11T11:51:43.9412743Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T11:51:43.9415127Z ##[command]git config --get-all http.proxy
2020-02-11T11:51:43.9463616Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69057/merge:refs/remotes/pull/69057/merge
---
2020-02-11T12:52:59.4988778Z .................................................................................................... 1700/9627
2020-02-11T12:53:04.9079619Z .................................................................................................... 1800/9627
2020-02-11T12:53:17.4749699Z ..............................i..................................................................... 1900/9627
2020-02-11T12:53:25.3642582Z .................................................................................................... 2000/9627
2020-02-11T12:53:40.3273807Z .................F..iiiii........................................................................... 2100/9627
2020-02-11T12:53:50.8251442Z .................................................................................................... 2300/9627
2020-02-11T12:53:53.4592195Z .................................................................................................... 2400/9627
2020-02-11T12:53:58.6168180Z .................................................................................................... 2500/9627
2020-02-11T12:54:20.7372534Z .................................................................................................... 2600/9627
---
2020-02-11T12:57:08.8083994Z .......................................................................i...............i............ 4900/9627
2020-02-11T12:57:16.9002933Z .................................................................................................... 5000/9627
2020-02-11T12:57:25.5038185Z .................................................................................................... 5100/9627
2020-02-11T12:57:30.4752896Z .............i...................................................................................... 5200/9627
2020-02-11T12:57:42.4198999Z .......................................................................................ii.ii........ 5300/9627
2020-02-11T12:57:46.3527874Z i...i............................................................................................... 5400/9627
2020-02-11T12:57:58.8543231Z .................................................................................................... 5600/9627
2020-02-11T12:58:07.7814105Z ...........................................................................i........................ 5700/9627
2020-02-11T12:58:15.5781889Z ...........................................................................F........................ 5800/9627
2020-02-11T12:58:22.3652792Z .................................................................................................... 5900/9627
2020-02-11T12:58:22.3652792Z .................................................................................................... 5900/9627
2020-02-11T12:58:32.5040497Z ...................................................................ii...i..ii...........i........... 6000/9627
2020-02-11T12:58:54.4062554Z .................................................................................................... 6200/9627
2020-02-11T12:59:02.3099410Z .................................................................................................... 6300/9627
2020-02-11T12:59:10.1450982Z ...............................................................................................i..ii 6400/9627
2020-02-11T12:59:23.8624089Z .................................................................................................... 6500/9627
---
2020-02-11T13:01:33.0895527Z .................................................................................................... 7600/9627
2020-02-11T13:01:37.3378677Z .................................................................................................... 7700/9627
2020-02-11T13:01:43.0959366Z .................................................................................................... 7800/9627
2020-02-11T13:01:51.6360883Z .................................................................................................... 7900/9627
2020-02-11T13:02:00.7724561Z .....................................................................iiiiiii.i...................... 8000/9627
2020-02-11T13:02:17.0379504Z .........i......i................................................................................... 8200/9627
2020-02-11T13:02:22.6903403Z .................................................................................................... 8300/9627
2020-02-11T13:02:36.9143506Z .................................................................................................... 8400/9627
2020-02-11T13:02:46.7083809Z .................................................................................................... 8500/9627
---
2020-02-11T13:04:50.1619833Z - error: recursion limit reached while expanding `recurse!`
2020-02-11T13:04:50.1620031Z + error: recursion limit reached while expanding the macro `recurse!`
2020-02-11T13:04:50.1620431Z 2   --> $DIR/recursion_limit_macro.rs:10:31
2020-02-11T13:04:50.1620610Z 3    |
2020-02-11T13:04:50.1620771Z 4 LL |     ($t:tt $($tail:tt)*) => { recurse!($($tail)*) };
2020-02-11T13:04:50.1621009Z 
2020-02-11T13:04:50.1621150Z The actual stderr differed from the expected stderr.
2020-02-11T13:04:50.1621657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro/recursion_limit_macro.stderr
2020-02-11T13:04:50.1621657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro/recursion_limit_macro.stderr
2020-02-11T13:04:50.1622106Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T13:04:50.1622579Z To only update this specific test, also pass `--test-args did_you_mean/recursion_limit_macro.rs`
2020-02-11T13:04:50.1622886Z error: 1 errors occurred comparing output.
2020-02-11T13:04:50.1623030Z status: exit code: 1
2020-02-11T13:04:50.1623030Z status: exit code: 1
2020-02-11T13:04:50.1624053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/recursion_limit_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_macro/auxiliary" "-A" "unused"
2020-02-11T13:04:50.1624684Z ------------------------------------------
2020-02-11T13:04:50.1624844Z 
2020-02-11T13:04:50.1625229Z ------------------------------------------
2020-02-11T13:04:50.1625410Z stderr:
2020-02-11T13:04:50.1625410Z stderr:
2020-02-11T13:04:50.1625756Z ------------------------------------------
2020-02-11T13:04:50.1625962Z error: recursion limit reached while expanding the macro `recurse!`
2020-02-11T13:04:50.1626359Z   --> /checkout/src/test/ui/did_you_mean/recursion_limit_macro.rs:10:31
2020-02-11T13:04:50.1626558Z    |
2020-02-11T13:04:50.1626705Z LL |     ($t:tt $($tail:tt)*) => { recurse!($($tail)*) }; //~ ERROR recursion limit
2020-02-11T13:04:50.1627002Z ...
2020-02-11T13:04:50.1627002Z ...
2020-02-11T13:04:50.1627140Z LL |     recurse!(0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9);
2020-02-11T13:04:50.1629589Z    |
2020-02-11T13:04:50.1629589Z    |
2020-02-11T13:04:50.1630015Z    = help: consider adding a `#![recursion_limit="20"]` attribute to your crate (`recursion_limit_macro`)
2020-02-11T13:04:50.1630780Z 
2020-02-11T13:04:50.1630924Z error: aborting due to previous error
2020-02-11T13:04:50.1631044Z 
2020-02-11T13:04:50.1631178Z 
---
2020-02-11T13:04:50.1632895Z - error: recursion limit reached while expanding `recursive!`
2020-02-11T13:04:50.1633081Z + error: recursion limit reached while expanding the macro `recursive!`
2020-02-11T13:04:50.1633608Z 2   --> $DIR/infinite-macro-expansion.rs:2:12
2020-02-11T13:04:50.1633843Z 3    |
2020-02-11T13:04:50.1633985Z 4 LL |     () => (recursive!())
2020-02-11T13:04:50.1634239Z 
2020-02-11T13:04:50.1634376Z The actual stderr differed from the expected stderr.
2020-02-11T13:04:50.1634875Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion/infinite-macro-expansion.stderr
2020-02-11T13:04:50.1634875Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion/infinite-macro-expansion.stderr
2020-02-11T13:04:50.1635324Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T13:04:50.1635824Z To only update this specific test, also pass `--test-args infinite/infinite-macro-expansion.rs`
2020-02-11T13:04:50.1640204Z error: 1 errors occurred comparing output.
2020-02-11T13:04:50.1640358Z status: exit code: 1
2020-02-11T13:04:50.1640358Z status: exit code: 1
2020-02-11T13:04:50.1651592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-macro-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-macro-expansion/auxiliary" "-A" "unused"
2020-02-11T13:04:50.1652200Z ------------------------------------------
2020-02-11T13:04:50.1652238Z 
2020-02-11T13:04:50.1652475Z ------------------------------------------
2020-02-11T13:04:50.1652521Z stderr:
2020-02-11T13:04:50.1652521Z stderr:
2020-02-11T13:04:50.1652763Z ------------------------------------------
2020-02-11T13:04:50.1652817Z error: recursion limit reached while expanding the macro `recursive!`
2020-02-11T13:04:50.1653080Z   --> /checkout/src/test/ui/infinite/infinite-macro-expansion.rs:2:12
2020-02-11T13:04:50.1653158Z    |
2020-02-11T13:04:50.1653218Z LL |     () => (recursive!()) //~ ERROR recursion limit reached while expanding `recursive!`
2020-02-11T13:04:50.1653328Z ...
2020-02-11T13:04:50.1653371Z LL |     recursive!()
2020-02-11T13:04:50.1653610Z    |     ------------ in this macro invocation
2020-02-11T13:04:50.1653672Z    |
2020-02-11T13:04:50.1653672Z    |
2020-02-11T13:04:50.1653724Z    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_macro_expansion`)
2020-02-11T13:04:50.1654077Z 
2020-02-11T13:04:50.1654122Z error: aborting due to previous error
2020-02-11T13:04:50.1654150Z 
2020-02-11T13:04:50.1654176Z 
---
2020-02-11T13:04:50.1655240Z - error: recursion limit reached while expanding `prob1!`
2020-02-11T13:04:50.1655294Z + error: recursion limit reached while expanding the macro `prob1!`
2020-02-11T13:04:50.1655533Z 2   --> $DIR/issue-16098.rs:7:18
2020-02-11T13:04:50.1655578Z 3    |
2020-02-11T13:04:50.1655799Z 4 LL |             $n + prob1!($n - 1);
2020-02-11T13:04:50.1655872Z 
2020-02-11T13:04:50.1655917Z The actual stderr differed from the expected stderr.
2020-02-11T13:04:50.1656225Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098/issue-16098.stderr
2020-02-11T13:04:50.1656225Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098/issue-16098.stderr
2020-02-11T13:04:50.1656483Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T13:04:50.1656774Z To only update this specific test, also pass `--test-args issues/issue-16098.rs`
2020-02-11T13:04:50.1656980Z error: 1 errors occurred comparing output.
2020-02-11T13:04:50.1657052Z status: exit code: 1
2020-02-11T13:04:50.1657052Z status: exit code: 1
2020-02-11T13:04:50.1657885Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16098.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16098/auxiliary" "-A" "unused"
2020-02-11T13:04:50.1658225Z ------------------------------------------
2020-02-11T13:04:50.1658258Z 
2020-02-11T13:04:50.1658502Z ------------------------------------------
2020-02-11T13:04:50.1658548Z stderr:
2020-02-11T13:04:50.1658548Z stderr:
2020-02-11T13:04:50.1658770Z ------------------------------------------
2020-02-11T13:04:50.1658845Z error: recursion limit reached while expanding the macro `prob1!`
2020-02-11T13:04:50.1659104Z   --> /checkout/src/test/ui/issues/issue-16098.rs:7:18
2020-02-11T13:04:50.1659154Z    |
2020-02-11T13:04:50.1661093Z LL |             $n + prob1!($n - 1); //~ ERROR recursion limit reached while expanding `prob1!`
2020-02-11T13:04:50.1679132Z ...
2020-02-11T13:04:50.1679132Z ...
2020-02-11T13:04:50.1679194Z LL |     println!("Problem 1: {}", prob1!(1000));
2020-02-11T13:04:50.1679692Z    |
2020-02-11T13:04:50.1679692Z    |
2020-02-11T13:04:50.1679744Z    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`issue_16098`)
2020-02-11T13:04:50.1680111Z 
2020-02-11T13:04:50.1680157Z error: aborting due to previous error
2020-02-11T13:04:50.1680202Z 
2020-02-11T13:04:50.1680228Z 
2020-02-11T13:04:50.1680228Z 
2020-02-11T13:04:50.1680472Z ------------------------------------------
2020-02-11T13:04:50.1680517Z 
2020-02-11T13:04:50.1680543Z 
2020-02-11T13:04:50.1680783Z ---- [ui] ui/macros/trace_faulty_macros.rs stdout ----
2020-02-11T13:04:50.1680831Z diff of stderr:
2020-02-11T13:04:50.1680876Z 
2020-02-11T13:04:50.1680919Z 22    = note: to `my_faulty_macro ! (bcd) ;`
2020-02-11T13:04:50.1680967Z 23    = note: expanding `my_faulty_macro! { bcd }`
2020-02-11T13:04:50.1682022Z - error: recursion limit reached while expanding `my_recursive_macro!`
2020-02-11T13:04:50.1682022Z - error: recursion limit reached while expanding `my_recursive_macro!`
2020-02-11T13:04:50.1682088Z + error: recursion limit reached while expanding the macro `my_recursive_macro!`
2020-02-11T13:04:50.1682431Z 27    |
2020-02-11T13:04:50.1682431Z 27    |
2020-02-11T13:04:50.1682474Z 28 LL |         my_recursive_macro!();
2020-02-11T13:04:50.1682529Z 
2020-02-11T13:04:50.1682596Z The actual stderr differed from the expected stderr.
2020-02-11T13:04:50.1683127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-02-11T13:04:50.1683127Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-02-11T13:04:50.1683389Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T13:04:50.1683721Z To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
2020-02-11T13:04:50.1683806Z error: 1 errors occurred comparing output.
2020-02-11T13:04:50.1683867Z status: exit code: 1
2020-02-11T13:04:50.1683867Z status: exit code: 1
2020-02-11T13:04:50.1684821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary" "-A" "unused"
2020-02-11T13:04:50.1685226Z ------------------------------------------
2020-02-11T13:04:50.1685263Z 
2020-02-11T13:04:50.1685525Z ------------------------------------------
2020-02-11T13:04:50.1685572Z stderr:
2020-02-11T13:04:50.1685572Z stderr:
2020-02-11T13:04:50.1685814Z ------------------------------------------
2020-02-11T13:04:50.1685875Z error: no rules expected the token `bcd`
2020-02-11T13:04:50.1686146Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:7:26
2020-02-11T13:04:50.1686197Z    |
2020-02-11T13:04:50.1686258Z LL | macro_rules! my_faulty_macro {
2020-02-11T13:04:50.1686527Z    | ---------------------------- when calling this macro
2020-02-11T13:04:50.1686577Z LL |     () => {
2020-02-11T13:04:50.1686625Z LL |         my_faulty_macro!(bcd); //~ ERROR no rules
2020-02-11T13:04:50.1686704Z    |                          ^^^ no rules expected this token in macro call
2020-02-11T13:04:50.1686760Z ...
2020-02-11T13:04:50.1686804Z LL |     my_faulty_macro!();
2020-02-11T13:04:50.1687131Z    |
2020-02-11T13:04:50.1687440Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-11T13:04:50.1687494Z 
2020-02-11T13:04:50.1687540Z note: trace_macro
2020-02-11T13:04:50.1687540Z note: trace_macro
2020-02-11T13:04:50.1687810Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:33:5
2020-02-11T13:04:50.1687872Z    |
2020-02-11T13:04:50.1687917Z LL |     my_faulty_macro!();
2020-02-11T13:04:50.1688004Z    |
2020-02-11T13:04:50.1688004Z    |
2020-02-11T13:04:50.1688065Z    = note: expanding `my_faulty_macro! {  }`
2020-02-11T13:04:50.1688114Z    = note: to `my_faulty_macro ! (bcd) ;`
2020-02-11T13:04:50.1688160Z    = note: expanding `my_faulty_macro! { bcd }`
2020-02-11T13:04:50.1688203Z 
2020-02-11T13:04:50.1688261Z error: recursion limit reached while expanding the macro `my_recursive_macro!`
2020-02-11T13:04:50.1688596Z    |
2020-02-11T13:04:50.1688596Z    |
2020-02-11T13:04:50.1688656Z LL |         my_recursive_macro!(); //~ ERROR recursion limit
2020-02-11T13:04:50.1688748Z ...
2020-02-11T13:04:50.1688748Z ...
2020-02-11T13:04:50.1688802Z LL |     my_recursive_macro!();
2020-02-11T13:04:50.1689111Z    |
2020-02-11T13:04:50.1689111Z    |
2020-02-11T13:04:50.1689173Z    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
2020-02-11T13:04:50.1689531Z 
2020-02-11T13:04:50.1689573Z note: trace_macro
2020-02-11T13:04:50.1689869Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:34:5
2020-02-11T13:04:50.1690013Z    |
2020-02-11T13:04:50.1690013Z    |
2020-02-11T13:04:50.1690056Z LL |     my_recursive_macro!();
2020-02-11T13:04:50.1690165Z    |
2020-02-11T13:04:50.1690165Z    |
2020-02-11T13:04:50.1690210Z    = note: expanding `my_recursive_macro! {  }`
2020-02-11T13:04:50.1690274Z    = note: to `my_recursive_macro ! () ;`
2020-02-11T13:04:50.1690322Z    = note: expanding `my_recursive_macro! {  }`
2020-02-11T13:04:50.1690369Z    = note: to `my_recursive_macro ! () ;`
2020-02-11T13:04:50.1690425Z    = note: expanding `my_recursive_macro! {  }`
2020-02-11T13:04:50.1690472Z    = note: to `my_recursive_macro ! () ;`
2020-02-11T13:04:50.1690520Z    = note: expanding `my_recursive_macro! {  }`
2020-02-11T13:04:50.1690567Z    = note: to `my_recursive_macro ! () ;`
2020-02-11T13:04:50.1690650Z error: aborting due to 2 previous errors
2020-02-11T13:04:50.1690680Z 
2020-02-11T13:04:50.1690707Z 
2020-02-11T13:04:50.1691072Z ------------------------------------------
---
2020-02-11T13:04:50.1692541Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-11T13:04:50.1692600Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-11T13:04:50.1692633Z 
2020-02-11T13:04:50.1692660Z 
2020-02-11T13:04:50.1694231Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-11T13:04:50.1694469Z 
2020-02-11T13:04:50.1694509Z 
2020-02-11T13:04:50.1695878Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-11T13:04:50.1697062Z Build completed unsuccessfully in 1:06:46
2020-02-11T13:04:50.1697062Z Build completed unsuccessfully in 1:06:46
2020-02-11T13:04:50.1725190Z == clock drift check ==
2020-02-11T13:04:50.1746643Z   local time: Tue Feb 11 13:04:50 UTC 2020
2020-02-11T13:04:50.3458355Z   network time: Tue, 11 Feb 2020 13:04:50 GMT
2020-02-11T13:04:50.3468397Z == end clock drift check ==
2020-02-11T13:04:50.8307809Z 
2020-02-11T13:04:50.8414497Z ##[error]Bash exited with code '1'.
2020-02-11T13:04:50.8428170Z ##[section]Finishing: Run build
2020-02-11T13:04:50.8450050Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69057/merge to s
2020-02-11T13:04:50.8452168Z Task         : Get sources
2020-02-11T13:04:50.8452221Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T13:04:50.8452274Z Version      : 1.0.0
2020-02-11T13:04:50.8452339Z Author       : Microsoft
2020-02-11T13:04:50.8452339Z Author       : Microsoft
2020-02-11T13:04:50.8452392Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T13:04:50.8452449Z ==============================================================================
2020-02-11T13:04:51.2796968Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T13:04:51.2839293Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69057/merge to s
2020-02-11T13:04:51.2953999Z Cleaning up task key
2020-02-11T13:04:51.2954768Z Start cleaning up orphan processes.
2020-02-11T13:04:51.3073480Z Terminate orphan process: pid (3449) (python)
2020-02-11T13:04:51.3315190Z ##[section]Finishing: Finalize Job

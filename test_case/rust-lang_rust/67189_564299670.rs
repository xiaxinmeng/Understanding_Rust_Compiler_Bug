plain
2019-12-10T21:55:12.7511448Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T21:55:13.4347957Z ##[command]git config gc.auto 0
2019-12-10T21:55:13.4353669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T21:55:13.4358050Z ##[command]git config --get-all http.proxy
2019-12-10T21:55:13.4363746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67189/merge:refs/remotes/pull/67189/merge
---
2019-12-10T22:52:30.4686431Z .................................................................................................... 1600/9342
2019-12-10T22:52:34.8171485Z .................................................................................................... 1700/9342
2019-12-10T22:52:46.4696700Z ..................................................i................................................. 1800/9342
2019-12-10T22:52:54.3507233Z .................................................................................................... 1900/9342
2019-12-10T22:53:07.7363126Z ...................................iiiii............................................................ 2000/9342
2019-12-10T22:53:18.4582395Z .................................................................................................... 2200/9342
2019-12-10T22:53:19.9455689Z .................................................................................................... 2300/9342
2019-12-10T22:53:24.2058752Z .................................................................................................... 2400/9342
2019-12-10T22:53:45.0362684Z .................................................................................................... 2500/9342
---
2019-12-10T22:56:21.1909668Z ......................................i...............i............................................. 4800/9342
2019-12-10T22:56:30.5722228Z .................................................................................................... 4900/9342
2019-12-10T22:56:36.6828192Z ..................................................................................i................. 5000/9342
2019-12-10T22:56:42.5374213Z .................................................................................................... 5100/9342
2019-12-10T22:56:51.6234242Z ................................................ii.ii...........i................................... 5200/9342
2019-12-10T22:57:00.4601565Z .................................................................................................... 5400/9342
2019-12-10T22:57:10.0723718Z .................................................................................................... 5500/9342
2019-12-10T22:57:16.9626186Z ..............................i..................................................................... 5600/9342
2019-12-10T22:57:23.0060605Z .................................................................................................... 5700/9342
2019-12-10T22:57:23.0060605Z .................................................................................................... 5700/9342
2019-12-10T22:57:34.1758749Z .................................................................................................... 5800/9342
2019-12-10T22:57:44.4290243Z .................ii...i..ii...........i............................................................. 5900/9342
2019-12-10T22:58:01.5640057Z .................................................................................................... 6100/9342
2019-12-10T22:58:09.4246474Z .................................................................................................... 6200/9342
2019-12-10T22:58:09.4246474Z .................................................................................................... 6200/9342
2019-12-10T22:58:26.6487524Z .........................................i..ii...................................................... 6300/9342
2019-12-10T22:58:47.5664542Z .................................................................................................... 6500/9342
2019-12-10T22:58:49.6217178Z .............i...................................................................................... 6600/9342
2019-12-10T22:58:52.0248004Z .................................................................................................... 6700/9342
2019-12-10T22:58:54.5049092Z ....i............................................................................................... 6800/9342
---
2019-12-10T23:00:29.7045942Z .................................................................................................... 7400/9342
2019-12-10T23:00:34.6543657Z .................................................................................................... 7500/9342
2019-12-10T23:00:41.6114688Z .................................................................................................... 7600/9342
2019-12-10T23:00:51.6648133Z .................................................................................................... 7700/9342
2019-12-10T23:00:57.9231943Z ....................iiii............................................................................ 7800/9342
2019-12-10T23:01:11.7716556Z ..........................................F......................................................... 8000/9342
2019-12-10T23:01:21.9578608Z ...............................................................................F.................... 8100/9342
2019-12-10T23:01:34.7116370Z .................................................................................................... 8200/9342
2019-12-10T23:01:41.6457990Z .................................................................................................... 8300/9342
---
2019-12-10T23:03:31.6995519Z failures:
2019-12-10T23:03:31.7000074Z 
2019-12-10T23:03:31.7000839Z ---- [ui] ui/autoderef-full-lval.rs stdout ----
2019-12-10T23:03:31.7001177Z 
2019-12-10T23:03:31.7001801Z error: /checkout/src/test/ui/autoderef-full-lval.rs:15: unexpected error: '15:24: 15:25: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>` [E0369]'
2019-12-10T23:03:31.7002073Z 
2019-12-10T23:03:31.7002708Z error: /checkout/src/test/ui/autoderef-full-lval.rs:21: unexpected error: '21:33: 21:34: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>` [E0369]'
2019-12-10T23:03:31.7002991Z 
2019-12-10T23:03:31.7003550Z error: /checkout/src/test/ui/autoderef-full-lval.rs:15: expected error not found: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T23:03:31.7003838Z 
2019-12-10T23:03:31.7004387Z error: /checkout/src/test/ui/autoderef-full-lval.rs:21: expected error not found: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T23:03:31.7005165Z error: 2 unexpected errors found, 2 expected errors not found
2019-12-10T23:03:31.7005465Z status: exit code: 1
2019-12-10T23:03:31.7005465Z status: exit code: 1
2019-12-10T23:03:31.7006719Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/autoderef-full-lval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/autoderef-full-lval/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7015355Z     Error {
2019-12-10T23:03:31.7015404Z         line_num: 15,
2019-12-10T23:03:31.7015446Z         kind: Some(
2019-12-10T23:03:31.7015508Z             Error,
2019-12-10T23:03:31.7015508Z             Error,
2019-12-10T23:03:31.7015548Z         ),
2019-12-10T23:03:31.7015599Z         msg: "15:24: 15:25: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>` [E0369]",
2019-12-10T23:03:31.7015710Z     Error {
2019-12-10T23:03:31.7015750Z         line_num: 21,
2019-12-10T23:03:31.7015812Z         kind: Some(
2019-12-10T23:03:31.7015854Z             Error,
2019-12-10T23:03:31.7015854Z             Error,
2019-12-10T23:03:31.7015894Z         ),
2019-12-10T23:03:31.7015957Z         msg: "21:33: 21:34: cannot add `std::boxed::Box<isize>` to `std::boxed::Box<isize>` [E0369]",
2019-12-10T23:03:31.7016070Z ]
2019-12-10T23:03:31.7016098Z 
2019-12-10T23:03:31.7016141Z not found errors (from test file): [
2019-12-10T23:03:31.7016203Z     Error {
2019-12-10T23:03:31.7016203Z     Error {
2019-12-10T23:03:31.7016246Z         line_num: 15,
2019-12-10T23:03:31.7016287Z         kind: Some(
2019-12-10T23:03:31.7016347Z             Error,
2019-12-10T23:03:31.7016387Z         ),
2019-12-10T23:03:31.7016435Z         msg: "binary operation `+` cannot be applied to type `std::boxed::Box<isize>`",
2019-12-10T23:03:31.7016538Z     Error {
2019-12-10T23:03:31.7016579Z         line_num: 21,
2019-12-10T23:03:31.7016620Z         kind: Some(
2019-12-10T23:03:31.7016681Z             Error,
2019-12-10T23:03:31.7016681Z             Error,
2019-12-10T23:03:31.7016720Z         ),
2019-12-10T23:03:31.7016766Z         msg: "binary operation `+` cannot be applied to type `std::boxed::Box<isize>`",
2019-12-10T23:03:31.7016875Z ]
2019-12-10T23:03:31.7016901Z 
2019-12-10T23:03:31.7017338Z thread '[ui] ui/autoderef-full-lval.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7017435Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T23:03:31.7017435Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-10T23:03:31.7017471Z 
2019-12-10T23:03:31.7017707Z ---- [ui] ui/binary-op-on-double-ref.rs stdout ----
2019-12-10T23:03:31.7017740Z 
2019-12-10T23:03:31.7018186Z error: /checkout/src/test/ui/binary-op-on-double-ref.rs:4: unexpected error: '4:11: 4:12: cannot mod `&&{integer}` by `{integer}` [E0369]'
2019-12-10T23:03:31.7018225Z 
2019-12-10T23:03:31.7018523Z error: /checkout/src/test/ui/binary-op-on-double-ref.rs:4: expected error not found: binary operation `%` cannot be applied to type `&&{integer}`
2019-12-10T23:03:31.7018622Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7018665Z status: exit code: 1
2019-12-10T23:03:31.7018665Z status: exit code: 1
2019-12-10T23:03:31.7019388Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binary-op-on-double-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binary-op-on-double-ref/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7019544Z     Error {
2019-12-10T23:03:31.7019585Z         line_num: 4,
2019-12-10T23:03:31.7019623Z         kind: Some(
2019-12-10T23:03:31.7019681Z             Error,
2019-12-10T23:03:31.7019681Z             Error,
2019-12-10T23:03:31.7019718Z         ),
2019-12-10T23:03:31.7019763Z         msg: "4:11: 4:12: cannot mod `&&{integer}` by `{integer}` [E0369]",
2019-12-10T23:03:31.7019991Z ]
2019-12-10T23:03:31.7020016Z 
2019-12-10T23:03:31.7020055Z not found errors (from test file): [
2019-12-10T23:03:31.7020171Z     Error {
2019-12-10T23:03:31.7020171Z     Error {
2019-12-10T23:03:31.7020209Z         line_num: 4,
2019-12-10T23:03:31.7020248Z         kind: Some(
2019-12-10T23:03:31.7020305Z             Error,
2019-12-10T23:03:31.7020342Z         ),
2019-12-10T23:03:31.7020385Z         msg: "binary operation `%` cannot be applied to type `&&{integer}`",
2019-12-10T23:03:31.7020482Z ]
2019-12-10T23:03:31.7020506Z 
2019-12-10T23:03:31.7020839Z thread '[ui] ui/binary-op-on-double-ref.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7020879Z 
2019-12-10T23:03:31.7020879Z 
2019-12-10T23:03:31.7021113Z ---- [ui] ui/binop/binop-bitxor-str.rs stdout ----
2019-12-10T23:03:31.7021144Z 
2019-12-10T23:03:31.7021383Z error: error pattern '`^` cannot be applied to type `std::string::String`' not found!
2019-12-10T23:03:31.7021449Z status: exit code: 1
2019-12-10T23:03:31.7022180Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-bitxor-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-bitxor-str/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7022510Z ------------------------------------------
2019-12-10T23:03:31.7022541Z 
2019-12-10T23:03:31.7022765Z ------------------------------------------
2019-12-10T23:03:31.7022807Z stderr:
2019-12-10T23:03:31.7022807Z stderr:
2019-12-10T23:03:31.7023006Z ------------------------------------------
2019-12-10T23:03:31.7023075Z error[E0369]: no implementation for `std::string::String ^ std::string::String`
2019-12-10T23:03:31.7023621Z    |
2019-12-10T23:03:31.7023621Z    |
2019-12-10T23:03:31.7023686Z LL | fn main() { let x = "a".to_string() ^ "b".to_string(); }
2019-12-10T23:03:31.7023969Z    |                     --------------- ^ --------------- std::string::String
2019-12-10T23:03:31.7024059Z    |                     std::string::String
2019-12-10T23:03:31.7024120Z    |
2019-12-10T23:03:31.7024167Z    = note: an implementation of `std::ops::BitXor` might be missing for `std::string::String`
2019-12-10T23:03:31.7024200Z 
---
2019-12-10T23:03:31.7024798Z 
2019-12-10T23:03:31.7024821Z 
2019-12-10T23:03:31.7025334Z ---- [ui] ui/binop/binop-mul-bool.rs stdout ----
2019-12-10T23:03:31.7025375Z 
2019-12-10T23:03:31.7025657Z error: error pattern '`*` cannot be applied to type `bool`' not found!
2019-12-10T23:03:31.7025715Z status: exit code: 1
2019-12-10T23:03:31.7026454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-mul-bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-mul-bool/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7026782Z ------------------------------------------
2019-12-10T23:03:31.7026839Z 
2019-12-10T23:03:31.7027059Z ------------------------------------------
2019-12-10T23:03:31.7027231Z stderr:
2019-12-10T23:03:31.7027231Z stderr:
2019-12-10T23:03:31.7027520Z ------------------------------------------
2019-12-10T23:03:31.7027571Z error[E0369]: cannot multiply `bool` to `bool`
2019-12-10T23:03:31.7027939Z   --> /checkout/src/test/ui/binop/binop-mul-bool.rs:3:26
2019-12-10T23:03:31.7028009Z    |
2019-12-10T23:03:31.7028169Z LL | fn main() { let x = true * false; }
2019-12-10T23:03:31.7028376Z    |                     ---- ^ ----- bool
2019-12-10T23:03:31.7028481Z    |                     bool
2019-12-10T23:03:31.7028519Z    |
2019-12-10T23:03:31.7028564Z    = note: an implementation of `std::ops::Mul` might be missing for `bool`
2019-12-10T23:03:31.7028615Z 
---
2019-12-10T23:03:31.7029206Z 
2019-12-10T23:03:31.7029230Z 
2019-12-10T23:03:31.7029457Z ---- [ui] ui/binop/binop-typeck.rs stdout ----
2019-12-10T23:03:31.7029495Z 
2019-12-10T23:03:31.7029783Z error: /checkout/src/test/ui/binop/binop-typeck.rs:6: unexpected error: '6:15: 6:16: cannot add `{integer}` to `bool` [E0369]'
2019-12-10T23:03:31.7029821Z 
2019-12-10T23:03:31.7030131Z error: /checkout/src/test/ui/binop/binop-typeck.rs:6: expected error not found: binary operation `+` cannot be applied to type `bool`
2019-12-10T23:03:31.7030210Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7030270Z status: exit code: 1
2019-12-10T23:03:31.7030270Z status: exit code: 1
2019-12-10T23:03:31.7030957Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-typeck/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7031111Z     Error {
2019-12-10T23:03:31.7031151Z         line_num: 6,
2019-12-10T23:03:31.7031211Z         kind: Some(
2019-12-10T23:03:31.7031249Z             Error,
2019-12-10T23:03:31.7031249Z             Error,
2019-12-10T23:03:31.7031287Z         ),
2019-12-10T23:03:31.7031330Z         msg: "6:15: 6:16: cannot add `{integer}` to `bool` [E0369]",
2019-12-10T23:03:31.7031430Z ]
2019-12-10T23:03:31.7031454Z 
2019-12-10T23:03:31.7031511Z not found errors (from test file): [
2019-12-10T23:03:31.7031552Z     Error {
2019-12-10T23:03:31.7031552Z     Error {
2019-12-10T23:03:31.7031590Z         line_num: 6,
2019-12-10T23:03:31.7031628Z         kind: Some(
2019-12-10T23:03:31.7031685Z             Error,
2019-12-10T23:03:31.7031723Z         ),
2019-12-10T23:03:31.7031771Z         msg: "binary operation `+` cannot be applied to type `bool`",
2019-12-10T23:03:31.7031875Z ]
2019-12-10T23:03:31.7031899Z 
2019-12-10T23:03:31.7032184Z thread '[ui] ui/binop/binop-typeck.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7032241Z 
2019-12-10T23:03:31.7032241Z 
2019-12-10T23:03:31.7032455Z ---- [ui] ui/for/for-loop-type-error.rs stdout ----
2019-12-10T23:03:31.7032485Z 
2019-12-10T23:03:31.7032782Z error: /checkout/src/test/ui/for/for-loop-type-error.rs:2: unexpected error: '2:16: 2:17: cannot add `()` to `()` [E0369]'
2019-12-10T23:03:31.7033074Z error: /checkout/src/test/ui/for/for-loop-type-error.rs:2: expected error not found: binary operation
2019-12-10T23:03:31.7033109Z 
2019-12-10T23:03:31.7033168Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7033210Z status: exit code: 1
2019-12-10T23:03:31.7033210Z status: exit code: 1
2019-12-10T23:03:31.7033993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for/for-loop-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-type-error/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7034213Z     Error {
2019-12-10T23:03:31.7034276Z         line_num: 2,
2019-12-10T23:03:31.7034317Z         kind: Some(
2019-12-10T23:03:31.7034358Z             Error,
2019-12-10T23:03:31.7034358Z             Error,
2019-12-10T23:03:31.7034418Z         ),
2019-12-10T23:03:31.7034462Z         msg: "2:16: 2:17: cannot add `()` to `()` [E0369]",
2019-12-10T23:03:31.7034563Z ]
2019-12-10T23:03:31.7034590Z 
2019-12-10T23:03:31.7034638Z not found errors (from test file): [
2019-12-10T23:03:31.7034681Z     Error {
---
2019-12-10T23:03:31.7035669Z thread '[ui] ui/for/for-loop-type-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7035711Z 
2019-12-10T23:03:31.7035938Z ---- [ui] ui/issues/issue-14915.rs stdout ----
2019-12-10T23:03:31.7035991Z 
2019-12-10T23:03:31.7036318Z error: /checkout/src/test/ui/issues/issue-14915.rs:6: unexpected error: '6:22: 6:23: cannot add `{integer}` to `std::boxed::Box<isize>` [E0369]'
2019-12-10T23:03:31.7036358Z 
2019-12-10T23:03:31.7036693Z error: /checkout/src/test/ui/issues/issue-14915.rs:6: expected error not found: binary operation `+` cannot be applied to type `std::boxed::Box<isize>`
2019-12-10T23:03:31.7036815Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7036859Z status: exit code: 1
2019-12-10T23:03:31.7036859Z status: exit code: 1
2019-12-10T23:03:31.7037617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14915.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14915/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7037770Z     Error {
2019-12-10T23:03:31.7037813Z         line_num: 6,
2019-12-10T23:03:31.7037863Z         kind: Some(
2019-12-10T23:03:31.7038024Z             Error,
2019-12-10T23:03:31.7038024Z             Error,
2019-12-10T23:03:31.7038063Z         ),
2019-12-10T23:03:31.7038231Z         msg: "6:22: 6:23: cannot add `{integer}` to `std::boxed::Box<isize>` [E0369]",
2019-12-10T23:03:31.7038331Z ]
2019-12-10T23:03:31.7038356Z 
2019-12-10T23:03:31.7038413Z not found errors (from test file): [
2019-12-10T23:03:31.7038453Z     Error {
2019-12-10T23:03:31.7038453Z     Error {
2019-12-10T23:03:31.7038491Z         line_num: 6,
2019-12-10T23:03:31.7038529Z         kind: Some(
2019-12-10T23:03:31.7038589Z             Error,
2019-12-10T23:03:31.7038626Z         ),
2019-12-10T23:03:31.7038670Z         msg: "binary operation `+` cannot be applied to type `std::boxed::Box<isize>`",
2019-12-10T23:03:31.7038766Z ]
2019-12-10T23:03:31.7038789Z 
2019-12-10T23:03:31.7039078Z thread '[ui] ui/issues/issue-14915.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7039136Z 
2019-12-10T23:03:31.7039136Z 
2019-12-10T23:03:31.7039450Z ---- [ui] ui/issues/issue-24363.rs stdout ----
2019-12-10T23:03:31.7039490Z 
2019-12-10T23:03:31.7039802Z error: /checkout/src/test/ui/issues/issue-24363.rs:3: unexpected error: '3:11: 3:12: cannot add `()` to `()` [E0369]'
2019-12-10T23:03:31.7039943Z 
2019-12-10T23:03:31.7040245Z error: /checkout/src/test/ui/issues/issue-24363.rs:3: expected error not found: binary operation `+` cannot be applied
2019-12-10T23:03:31.7040342Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7040383Z status: exit code: 1
2019-12-10T23:03:31.7040383Z status: exit code: 1
2019-12-10T23:03:31.7041076Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24363/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7041232Z     Error {
2019-12-10T23:03:31.7041292Z         line_num: 3,
2019-12-10T23:03:31.7041330Z         kind: Some(
2019-12-10T23:03:31.7041368Z             Error,
2019-12-10T23:03:31.7041368Z             Error,
2019-12-10T23:03:31.7041406Z         ),
2019-12-10T23:03:31.7041467Z         msg: "3:11: 3:12: cannot add `()` to `()` [E0369]",
2019-12-10T23:03:31.7041546Z ]
2019-12-10T23:03:31.7041589Z 
2019-12-10T23:03:31.7041628Z not found errors (from test file): [
2019-12-10T23:03:31.7041667Z     Error {
2019-12-10T23:03:31.7041667Z     Error {
2019-12-10T23:03:31.7041705Z         line_num: 3,
2019-12-10T23:03:31.7041761Z         kind: Some(
2019-12-10T23:03:31.7041799Z             Error,
2019-12-10T23:03:31.7041880Z         ),
2019-12-10T23:03:31.7041942Z         msg: "binary operation `+` cannot be applied",
2019-12-10T23:03:31.7042025Z ]
2019-12-10T23:03:31.7042048Z 
2019-12-10T23:03:31.7042358Z thread '[ui] ui/issues/issue-24363.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7042403Z 
2019-12-10T23:03:31.7042403Z 
2019-12-10T23:03:31.7042615Z ---- [ui] ui/issues/issue-28837.rs stdout ----
2019-12-10T23:03:31.7042665Z 
2019-12-10T23:03:31.7042937Z error: /checkout/src/test/ui/issues/issue-28837.rs:6: unexpected error: '6:7: 6:8: cannot add `A` to `A` [E0369]'
2019-12-10T23:03:31.7042971Z 
2019-12-10T23:03:31.7043251Z error: /checkout/src/test/ui/issues/issue-28837.rs:8: unexpected error: '8:7: 8:8: cannot substract `A` from `A` [E0369]'
2019-12-10T23:03:31.7043304Z 
2019-12-10T23:03:31.7043584Z error: /checkout/src/test/ui/issues/issue-28837.rs:10: unexpected error: '10:7: 10:8: cannot multiply `A` to `A` [E0369]'
2019-12-10T23:03:31.7043620Z 
2019-12-10T23:03:31.7043923Z error: /checkout/src/test/ui/issues/issue-28837.rs:12: unexpected error: '12:7: 12:8: cannot divide `A` by `A` [E0369]'
2019-12-10T23:03:31.7043959Z 
2019-12-10T23:03:31.7044238Z error: /checkout/src/test/ui/issues/issue-28837.rs:14: unexpected error: '14:7: 14:8: cannot mod `A` by `A` [E0369]'
2019-12-10T23:03:31.7044282Z 
2019-12-10T23:03:31.7044583Z error: /checkout/src/test/ui/issues/issue-28837.rs:16: unexpected error: '16:7: 16:8: no implementation for `A & A` [E0369]'
2019-12-10T23:03:31.7044619Z 
2019-12-10T23:03:31.7048502Z error: /checkout/src/test/ui/issues/issue-28837.rs:18: unexpected error: '18:7: 18:8: no implementation for `A | A` [E0369]'
2019-12-10T23:03:31.7048559Z 
2019-12-10T23:03:31.7048960Z error: /checkout/src/test/ui/issues/issue-28837.rs:20: unexpected error: '20:7: 20:9: no implementation for `A << A [E0369]'
2019-12-10T23:03:31.7048998Z 
2019-12-10T23:03:31.7049302Z error: /checkout/src/test/ui/issues/issue-28837.rs:22: unexpected error: '22:7: 22:9: no implementation for `A >> A [E0369]'
2019-12-10T23:03:31.7049340Z 
2019-12-10T23:03:31.7050033Z error: /checkout/src/test/ui/issues/issue-28837.rs:6: expected error not found: binary operation `+` cannot be applied to type `A`
2019-12-10T23:03:31.7050507Z error: /checkout/src/test/ui/issues/issue-28837.rs:8: expected error not found: binary operation `-` cannot be applied to type `A`
2019-12-10T23:03:31.7050572Z 
2019-12-10T23:03:31.7050572Z 
2019-12-10T23:03:31.7050883Z error: /checkout/src/test/ui/issues/issue-28837.rs:10: expected error not found: binary operation `*` cannot be applied to type `A`
2019-12-10T23:03:31.7050921Z 
2019-12-10T23:03:31.7051225Z error: /checkout/src/test/ui/issues/issue-28837.rs:12: expected error not found: binary operation `/` cannot be applied to type `A`
2019-12-10T23:03:31.7051284Z 
2019-12-10T23:03:31.7051791Z error: /checkout/src/test/ui/issues/issue-28837.rs:14: expected error not found: binary operation `%` cannot be applied to type `A`
2019-12-10T23:03:31.7052139Z error: /checkout/src/test/ui/issues/issue-28837.rs:16: expected error not found: binary operation `&` cannot be applied to type `A`
2019-12-10T23:03:31.7052176Z 
2019-12-10T23:03:31.7052176Z 
2019-12-10T23:03:31.7052462Z error: /checkout/src/test/ui/issues/issue-28837.rs:18: expected error not found: binary operation `|` cannot be applied to type `A`
2019-12-10T23:03:31.7052506Z 
2019-12-10T23:03:31.7052811Z error: /checkout/src/test/ui/issues/issue-28837.rs:20: expected error not found: binary operation `<<` cannot be applied to type `A`
2019-12-10T23:03:31.7052848Z 
2019-12-10T23:03:31.7053134Z error: /checkout/src/test/ui/issues/issue-28837.rs:22: expected error not found: binary operation `>>` cannot be applied to type `A`
2019-12-10T23:03:31.7053232Z error: 9 unexpected errors found, 9 expected errors not found
2019-12-10T23:03:31.7053275Z status: exit code: 1
2019-12-10T23:03:31.7053275Z status: exit code: 1
2019-12-10T23:03:31.7053965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28837.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28837" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28837/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7054120Z     Error {
2019-12-10T23:03:31.7054181Z         line_num: 6,
2019-12-10T23:03:31.7054220Z         kind: Some(
2019-12-10T23:03:31.7054257Z             Error,
2019-12-10T23:03:31.7054257Z             Error,
2019-12-10T23:03:31.7054315Z         ),
2019-12-10T23:03:31.7054356Z         msg: "6:7: 6:8: cannot add `A` to `A` [E0369]",
2019-12-10T23:03:31.7054454Z     Error {
2019-12-10T23:03:31.7054491Z         line_num: 8,
2019-12-10T23:03:31.7054528Z         kind: Some(
2019-12-10T23:03:31.7054565Z             Error,
2019-12-10T23:03:31.7054565Z             Error,
2019-12-10T23:03:31.7054623Z         ),
2019-12-10T23:03:31.7054672Z         msg: "8:7: 8:8: cannot substract `A` from `A` [E0369]",
2019-12-10T23:03:31.7054769Z     Error {
2019-12-10T23:03:31.7054815Z         line_num: 10,
2019-12-10T23:03:31.7054854Z         kind: Some(
2019-12-10T23:03:31.7055146Z             Error,
2019-12-10T23:03:31.7055146Z             Error,
2019-12-10T23:03:31.7055195Z         ),
2019-12-10T23:03:31.7055240Z         msg: "10:7: 10:8: cannot multiply `A` to `A` [E0369]",
2019-12-10T23:03:31.7055341Z     Error {
2019-12-10T23:03:31.7055383Z         line_num: 12,
2019-12-10T23:03:31.7055423Z         kind: Some(
2019-12-10T23:03:31.7055483Z             Error,
2019-12-10T23:03:31.7055483Z             Error,
2019-12-10T23:03:31.7055522Z         ),
2019-12-10T23:03:31.7055567Z         msg: "12:7: 12:8: cannot divide `A` by `A` [E0369]",
2019-12-10T23:03:31.7055668Z     Error {
2019-12-10T23:03:31.7055708Z         line_num: 14,
2019-12-10T23:03:31.7055749Z         kind: Some(
2019-12-10T23:03:31.7055809Z             Error,
2019-12-10T23:03:31.7055809Z             Error,
2019-12-10T23:03:31.7055848Z         ),
2019-12-10T23:03:31.7056010Z         msg: "14:7: 14:8: cannot mod `A` by `A` [E0369]",
2019-12-10T23:03:31.7056186Z     Error {
2019-12-10T23:03:31.7056227Z         line_num: 16,
2019-12-10T23:03:31.7056267Z         kind: Some(
2019-12-10T23:03:31.7056326Z             Error,
2019-12-10T23:03:31.7056326Z             Error,
2019-12-10T23:03:31.7056365Z         ),
2019-12-10T23:03:31.7056411Z         msg: "16:7: 16:8: no implementation for `A & A` [E0369]",
2019-12-10T23:03:31.7056513Z     Error {
2019-12-10T23:03:31.7056553Z         line_num: 18,
2019-12-10T23:03:31.7056594Z         kind: Some(
2019-12-10T23:03:31.7056655Z             Error,
2019-12-10T23:03:31.7056655Z             Error,
2019-12-10T23:03:31.7056694Z         ),
2019-12-10T23:03:31.7056739Z         msg: "18:7: 18:8: no implementation for `A | A` [E0369]",
2019-12-10T23:03:31.7056842Z     Error {
2019-12-10T23:03:31.7056882Z         line_num: 20,
2019-12-10T23:03:31.7056942Z         kind: Some(
2019-12-10T23:03:31.7056989Z             Error,
2019-12-10T23:03:31.7056989Z             Error,
2019-12-10T23:03:31.7057028Z         ),
2019-12-10T23:03:31.7057073Z         msg: "20:7: 20:9: no implementation for `A << A [E0369]",
2019-12-10T23:03:31.7057182Z     Error {
2019-12-10T23:03:31.7057223Z         line_num: 22,
2019-12-10T23:03:31.7057282Z         kind: Some(
2019-12-10T23:03:31.7057322Z             Error,
2019-12-10T23:03:31.7057322Z             Error,
2019-12-10T23:03:31.7057361Z         ),
2019-12-10T23:03:31.7057425Z         msg: "22:7: 22:9: no implementation for `A >> A [E0369]",
2019-12-10T23:03:31.7057506Z ]
2019-12-10T23:03:31.7057534Z 
2019-12-10T23:03:31.7057594Z not found errors (from test file): [
2019-12-10T23:03:31.7057636Z     Error {
2019-12-10T23:03:31.7057636Z     Error {
2019-12-10T23:03:31.7057677Z         line_num: 6,
2019-12-10T23:03:31.7057717Z         kind: Some(
2019-12-10T23:03:31.7057777Z             Error,
2019-12-10T23:03:31.7057815Z         ),
2019-12-10T23:03:31.7057860Z         msg: "binary operation `+` cannot be applied to type `A`",
2019-12-10T23:03:31.7057969Z     Error {
2019-12-10T23:03:31.7058008Z         line_num: 8,
2019-12-10T23:03:31.7058067Z         kind: Some(
2019-12-10T23:03:31.7058115Z             Error,
2019-12-10T23:03:31.7058115Z             Error,
2019-12-10T23:03:31.7058155Z         ),
2019-12-10T23:03:31.7058592Z         msg: "binary operation `-` cannot be applied to type `A`",
2019-12-10T23:03:31.7058696Z     Error {
2019-12-10T23:03:31.7058734Z         line_num: 10,
2019-12-10T23:03:31.7058791Z         kind: Some(
2019-12-10T23:03:31.7058829Z             Error,
2019-12-10T23:03:31.7058829Z             Error,
2019-12-10T23:03:31.7058865Z         ),
2019-12-10T23:03:31.7058907Z         msg: "binary operation `*` cannot be applied to type `A`",
2019-12-10T23:03:31.7059003Z     Error {
2019-12-10T23:03:31.7059040Z         line_num: 12,
2019-12-10T23:03:31.7059097Z         kind: Some(
2019-12-10T23:03:31.7059135Z             Error,
2019-12-10T23:03:31.7059135Z             Error,
2019-12-10T23:03:31.7059172Z         ),
2019-12-10T23:03:31.7059241Z         msg: "binary operation `/` cannot be applied to type `A`",
2019-12-10T23:03:31.7059318Z     Error {
2019-12-10T23:03:31.7059355Z         line_num: 14,
2019-12-10T23:03:31.7059410Z         kind: Some(
2019-12-10T23:03:31.7085309Z             Error,
2019-12-10T23:03:31.7085309Z             Error,
2019-12-10T23:03:31.7085405Z         ),
2019-12-10T23:03:31.7085455Z         msg: "binary operation `%` cannot be applied to type `A`",
2019-12-10T23:03:31.7085562Z     Error {
2019-12-10T23:03:31.7085604Z         line_num: 16,
2019-12-10T23:03:31.7085646Z         kind: Some(
2019-12-10T23:03:31.7085704Z             Error,
2019-12-10T23:03:31.7085704Z             Error,
2019-12-10T23:03:31.7085744Z         ),
2019-12-10T23:03:31.7085790Z         msg: "binary operation `&` cannot be applied to type `A`",
2019-12-10T23:03:31.7085882Z     Error {
2019-12-10T23:03:31.7085924Z         line_num: 18,
2019-12-10T23:03:31.7085965Z         kind: Some(
2019-12-10T23:03:31.7086024Z             Error,
2019-12-10T23:03:31.7086024Z             Error,
2019-12-10T23:03:31.7086064Z         ),
2019-12-10T23:03:31.7086273Z         msg: "binary operation `|` cannot be applied to type `A`",
2019-12-10T23:03:31.7086462Z     Error {
2019-12-10T23:03:31.7086503Z         line_num: 20,
2019-12-10T23:03:31.7086544Z         kind: Some(
2019-12-10T23:03:31.7086598Z             Error,
2019-12-10T23:03:31.7086598Z             Error,
2019-12-10T23:03:31.7086638Z         ),
2019-12-10T23:03:31.7086684Z         msg: "binary operation `<<` cannot be applied to type `A`",
2019-12-10T23:03:31.7086780Z     Error {
2019-12-10T23:03:31.7086820Z         line_num: 22,
2019-12-10T23:03:31.7086868Z         kind: Some(
2019-12-10T23:03:31.7086908Z             Error,
2019-12-10T23:03:31.7086908Z             Error,
2019-12-10T23:03:31.7086948Z         ),
2019-12-10T23:03:31.7086994Z         msg: "binary operation `>>` cannot be applied to type `A`",
2019-12-10T23:03:31.7087082Z ]
2019-12-10T23:03:31.7087112Z 
2019-12-10T23:03:31.7087616Z thread '[ui] ui/issues/issue-28837.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7087663Z 
2019-12-10T23:03:31.7087663Z 
2019-12-10T23:03:31.7088019Z ---- [ui] ui/issues/issue-31076.rs stdout ----
2019-12-10T23:03:31.7088060Z 
2019-12-10T23:03:31.7088484Z error: /checkout/src/test/ui/issues/issue-31076.rs:13: unexpected error: '13:15: 13:16: cannot add `{integer}` to `{integer}` [E0369]'
2019-12-10T23:03:31.7088521Z 
2019-12-10T23:03:31.7088809Z error: /checkout/src/test/ui/issues/issue-31076.rs:15: unexpected error: '15:18: 15:19: cannot add `i32` to `i32` [E0369]'
2019-12-10T23:03:31.7088844Z 
2019-12-10T23:03:31.7089154Z error: /checkout/src/test/ui/issues/issue-31076.rs:13: expected error not found: binary operation `+` cannot be applied to type `{integer}`
2019-12-10T23:03:31.7089191Z 
2019-12-10T23:03:31.7089476Z error: /checkout/src/test/ui/issues/issue-31076.rs:15: expected error not found: binary operation `+` cannot be applied to type `i32`
2019-12-10T23:03:31.7089574Z error: 2 unexpected errors found, 2 expected errors not found
2019-12-10T23:03:31.7089624Z status: exit code: 1
2019-12-10T23:03:31.7089624Z status: exit code: 1
2019-12-10T23:03:31.7090327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31076/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7090482Z     Error {
2019-12-10T23:03:31.7090523Z         line_num: 13,
2019-12-10T23:03:31.7090565Z         kind: Some(
2019-12-10T23:03:31.7090603Z             Error,
2019-12-10T23:03:31.7090603Z             Error,
2019-12-10T23:03:31.7090647Z         ),
2019-12-10T23:03:31.7090697Z         msg: "13:15: 13:16: cannot add `{integer}` to `{integer}` [E0369]",
2019-12-10T23:03:31.7090783Z     Error {
2019-12-10T23:03:31.7090821Z         line_num: 15,
2019-12-10T23:03:31.7090865Z         kind: Some(
2019-12-10T23:03:31.7090910Z             Error,
2019-12-10T23:03:31.7090910Z             Error,
2019-12-10T23:03:31.7090947Z         ),
2019-12-10T23:03:31.7090990Z         msg: "15:18: 15:19: cannot add `i32` to `i32` [E0369]",
2019-12-10T23:03:31.7091073Z ]
2019-12-10T23:03:31.7091098Z 
2019-12-10T23:03:31.7091136Z not found errors (from test file): [
2019-12-10T23:03:31.7091189Z     Error {
2019-12-10T23:03:31.7091189Z     Error {
2019-12-10T23:03:31.7091228Z         line_num: 13,
2019-12-10T23:03:31.7091267Z         kind: Some(
2019-12-10T23:03:31.7091305Z             Error,
2019-12-10T23:03:31.7091357Z         ),
2019-12-10T23:03:31.7091400Z         msg: "binary operation `+` cannot be applied to type `{integer}`",
2019-12-10T23:03:31.7091491Z     Error {
2019-12-10T23:03:31.7091528Z         line_num: 15,
2019-12-10T23:03:31.7091661Z         kind: Some(
2019-12-10T23:03:31.7091709Z             Error,
2019-12-10T23:03:31.7091709Z             Error,
2019-12-10T23:03:31.7091753Z         ),
2019-12-10T23:03:31.7091796Z         msg: "binary operation `+` cannot be applied to type `i32`",
2019-12-10T23:03:31.7091948Z ]
2019-12-10T23:03:31.7091972Z 
2019-12-10T23:03:31.7092306Z thread '[ui] ui/issues/issue-31076.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7092345Z 
2019-12-10T23:03:31.7092345Z 
2019-12-10T23:03:31.7092580Z ---- [ui] ui/issues/issue-35668.rs stdout ----
2019-12-10T23:03:31.7092611Z 
2019-12-10T23:03:31.7092893Z error: /checkout/src/test/ui/issues/issue-35668.rs:2: unexpected error: '2:23: 2:24: cannot multiply `&T` to `&T` [E0369]'
2019-12-10T23:03:31.7092940Z 
2019-12-10T23:03:31.7093226Z error: /checkout/src/test/ui/issues/issue-35668.rs:2: expected error not found: binary operation `*` cannot be applied to type `&T`
2019-12-10T23:03:31.7093313Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7093369Z status: exit code: 1
2019-12-10T23:03:31.7093369Z status: exit code: 1
2019-12-10T23:03:31.7094060Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35668.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35668/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7094203Z     Error {
2019-12-10T23:03:31.7094243Z         line_num: 2,
2019-12-10T23:03:31.7094289Z         kind: Some(
2019-12-10T23:03:31.7094326Z             Error,
2019-12-10T23:03:31.7094326Z             Error,
2019-12-10T23:03:31.7094364Z         ),
2019-12-10T23:03:31.7094419Z         msg: "2:23: 2:24: cannot multiply `&T` to `&T` [E0369]",
2019-12-10T23:03:31.7094499Z ]
2019-12-10T23:03:31.7094523Z 
2019-12-10T23:03:31.7094580Z not found errors (from test file): [
2019-12-10T23:03:31.7094625Z     Error {
2019-12-10T23:03:31.7094625Z     Error {
2019-12-10T23:03:31.7094663Z         line_num: 2,
2019-12-10T23:03:31.7094701Z         kind: Some(
2019-12-10T23:03:31.7094757Z             Error,
2019-12-10T23:03:31.7094793Z         ),
2019-12-10T23:03:31.7095069Z         msg: "binary operation `*` cannot be applied to type `&T`",
2019-12-10T23:03:31.7095182Z ]
2019-12-10T23:03:31.7095274Z 
2019-12-10T23:03:31.7095617Z thread '[ui] ui/issues/issue-35668.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7095665Z 
2019-12-10T23:03:31.7095665Z 
2019-12-10T23:03:31.7095897Z ---- [ui] ui/issues/issue-3820.rs stdout ----
2019-12-10T23:03:31.7095930Z 
2019-12-10T23:03:31.7096266Z error: /checkout/src/test/ui/issues/issue-3820.rs:14: unexpected error: '14:15: 14:16: cannot multiply `{integer}` to `Thing` [E0369]'
2019-12-10T23:03:31.7096308Z 
2019-12-10T23:03:31.7096623Z error: /checkout/src/test/ui/issues/issue-3820.rs:14: expected error not found: binary operation `*` cannot be applied to type `Thing`
2019-12-10T23:03:31.7096723Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7096768Z status: exit code: 1
2019-12-10T23:03:31.7096768Z status: exit code: 1
2019-12-10T23:03:31.7097504Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3820.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3820/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7097753Z     Error {
2019-12-10T23:03:31.7097824Z         line_num: 14,
2019-12-10T23:03:31.7097865Z         kind: Some(
2019-12-10T23:03:31.7098079Z             Error,
2019-12-10T23:03:31.7098079Z             Error,
2019-12-10T23:03:31.7098132Z         ),
2019-12-10T23:03:31.7098176Z         msg: "14:15: 14:16: cannot multiply `{integer}` to `Thing` [E0369]",
2019-12-10T23:03:31.7098752Z ]
2019-12-10T23:03:31.7098785Z 
2019-12-10T23:03:31.7098825Z not found errors (from test file): [
2019-12-10T23:03:31.7098863Z     Error {
2019-12-10T23:03:31.7098863Z     Error {
2019-12-10T23:03:31.7098907Z         line_num: 14,
2019-12-10T23:03:31.7098945Z         kind: Some(
2019-12-10T23:03:31.7100008Z             Error,
2019-12-10T23:03:31.7100067Z         ),
2019-12-10T23:03:31.7100134Z         msg: "binary operation `*` cannot be applied to type `Thing`",
2019-12-10T23:03:31.7100211Z ]
2019-12-10T23:03:31.7100247Z 
2019-12-10T23:03:31.7100664Z thread '[ui] ui/issues/issue-3820.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7100706Z 
2019-12-10T23:03:31.7100706Z 
2019-12-10T23:03:31.7100931Z ---- [ui] ui/issues/issue-40610.rs stdout ----
2019-12-10T23:03:31.7100981Z 
2019-12-10T23:03:31.7102975Z error: /checkout/src/test/ui/issues/issue-40610.rs:4: unexpected error: '4:8: 4:9: cannot add `()` to `()` [E0369]'
2019-12-10T23:03:31.7103026Z 
2019-12-10T23:03:31.7104613Z error: /checkout/src/test/ui/issues/issue-40610.rs:4: expected error not found: binary operation `+` cannot be applied to type `()`
2019-12-10T23:03:31.7104705Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7104748Z status: exit code: 1
2019-12-10T23:03:31.7104748Z status: exit code: 1
2019-12-10T23:03:31.7105809Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40610.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40610/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7105981Z     Error {
2019-12-10T23:03:31.7106031Z         line_num: 4,
2019-12-10T23:03:31.7106075Z         kind: Some(
2019-12-10T23:03:31.7106137Z             Error,
2019-12-10T23:03:31.7106137Z             Error,
2019-12-10T23:03:31.7106180Z         ),
2019-12-10T23:03:31.7106227Z         msg: "4:8: 4:9: cannot add `()` to `()` [E0369]",
2019-12-10T23:03:31.7106332Z ]
2019-12-10T23:03:31.7106361Z 
2019-12-10T23:03:31.7106406Z not found errors (from test file): [
2019-12-10T23:03:31.7106462Z     Error {
2019-12-10T23:03:31.7106462Z     Error {
2019-12-10T23:03:31.7106507Z         line_num: 4,
2019-12-10T23:03:31.7106550Z         kind: Some(
2019-12-10T23:03:31.7106601Z             Error,
2019-12-10T23:03:31.7106644Z         ),
2019-12-10T23:03:31.7106698Z         msg: "binary operation `+` cannot be applied to type `()`",
2019-12-10T23:03:31.7106802Z ]
2019-12-10T23:03:31.7106830Z 
2019-12-10T23:03:31.7107181Z thread '[ui] ui/issues/issue-40610.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7107231Z 
2019-12-10T23:03:31.7107231Z 
2019-12-10T23:03:31.7107488Z ---- [ui] ui/issues/issue-41394.rs stdout ----
2019-12-10T23:03:31.7107524Z 
2019-12-10T23:03:31.7107854Z error: /checkout/src/test/ui/issues/issue-41394.rs:2: unexpected error: '2:12: 2:13: cannot add `{integer}` to `&str` [E0369]'
2019-12-10T23:03:31.7108007Z 
2019-12-10T23:03:31.7108418Z error: /checkout/src/test/ui/issues/issue-41394.rs:2: expected error not found: binary operation `+` cannot be applied to type `&str`
2019-12-10T23:03:31.7108496Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7108555Z status: exit code: 1
2019-12-10T23:03:31.7108555Z status: exit code: 1
2019-12-10T23:03:31.7109379Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7109605Z     Error {
2019-12-10T23:03:31.7109650Z         line_num: 2,
2019-12-10T23:03:31.7109705Z         kind: Some(
2019-12-10T23:03:31.7109746Z             Error,
2019-12-10T23:03:31.7109746Z             Error,
2019-12-10T23:03:31.7109785Z         ),
2019-12-10T23:03:31.7109842Z         msg: "2:12: 2:13: cannot add `{integer}` to `&str` [E0369]",
2019-12-10T23:03:31.7109924Z ]
2019-12-10T23:03:31.7109974Z 
2019-12-10T23:03:31.7110016Z not found errors (from test file): [
2019-12-10T23:03:31.7110058Z     Error {
2019-12-10T23:03:31.7110058Z     Error {
2019-12-10T23:03:31.7110105Z         line_num: 2,
2019-12-10T23:03:31.7110157Z         kind: Some(
2019-12-10T23:03:31.7110197Z             Error,
2019-12-10T23:03:31.7110236Z         ),
2019-12-10T23:03:31.7110297Z         msg: "binary operation `+` cannot be applied to type `&str`",
2019-12-10T23:03:31.7110378Z ]
2019-12-10T23:03:31.7110404Z 
2019-12-10T23:03:31.7110763Z thread '[ui] ui/issues/issue-41394.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7110805Z 
2019-12-10T23:03:31.7110805Z 
2019-12-10T23:03:31.7111053Z ---- [ui] ui/or-patterns/or-patterns-syntactic-fail.rs stdout ----
2019-12-10T23:03:31.7111088Z 
2019-12-10T23:03:31.7111426Z error: /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:24: unexpected error: '24:22: 24:23: no implementation for `E | ()` [E0369]'
2019-12-10T23:03:31.7111466Z 
2019-12-10T23:03:31.7111803Z error: /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:24: expected error not found: binary operation `|` cannot be applied to type `E`
2019-12-10T23:03:31.7111901Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7111945Z status: exit code: 1
2019-12-10T23:03:31.7111945Z status: exit code: 1
2019-12-10T23:03:31.7112721Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7112883Z     Error {
2019-12-10T23:03:31.7112926Z         line_num: 24,
2019-12-10T23:03:31.7112966Z         kind: Some(
2019-12-10T23:03:31.7113026Z             Error,
2019-12-10T23:03:31.7113026Z             Error,
2019-12-10T23:03:31.7113065Z         ),
2019-12-10T23:03:31.7113109Z         msg: "24:22: 24:23: no implementation for `E | ()` [E0369]",
2019-12-10T23:03:31.7113201Z ]
2019-12-10T23:03:31.7113226Z 
2019-12-10T23:03:31.7113267Z not found errors (from test file): [
2019-12-10T23:03:31.7113317Z     Error {
2019-12-10T23:03:31.7113317Z     Error {
2019-12-10T23:03:31.7113357Z         line_num: 24,
2019-12-10T23:03:31.7113397Z         kind: Some(
2019-12-10T23:03:31.7113443Z             Error,
2019-12-10T23:03:31.7113483Z         ),
2019-12-10T23:03:31.7113527Z         msg: "binary operation `|` cannot be applied to type `E`",
2019-12-10T23:03:31.7113616Z ]
2019-12-10T23:03:31.7113642Z 
2019-12-10T23:03:31.7114065Z thread '[ui] ui/or-patterns/or-patterns-syntactic-fail.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7114133Z 
2019-12-10T23:03:31.7114133Z 
2019-12-10T23:03:31.7114412Z ---- [ui] ui/pattern/pattern-tyvar-2.rs stdout ----
2019-12-10T23:03:31.7114534Z 
2019-12-10T23:03:31.7115155Z error: /checkout/src/test/ui/pattern/pattern-tyvar-2.rs:3: unexpected error: '3:71: 3:72: cannot multiply `{integer}` to `std::vec::Vec<isize>` [E0369]'
2019-12-10T23:03:31.7115222Z 
2019-12-10T23:03:31.7144465Z error: /checkout/src/test/ui/pattern/pattern-tyvar-2.rs:3: expected error not found: binary operation `*` cannot be applied to
2019-12-10T23:03:31.7144611Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7144693Z status: exit code: 1
2019-12-10T23:03:31.7144693Z status: exit code: 1
2019-12-10T23:03:31.7146003Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-tyvar-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-tyvar-2/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7146213Z     Error {
2019-12-10T23:03:31.7146288Z         line_num: 3,
2019-12-10T23:03:31.7146335Z         kind: Some(
2019-12-10T23:03:31.7146383Z             Error,
2019-12-10T23:03:31.7146383Z             Error,
2019-12-10T23:03:31.7146429Z         ),
2019-12-10T23:03:31.7146504Z         msg: "3:71: 3:72: cannot multiply `{integer}` to `std::vec::Vec<isize>` [E0369]",
2019-12-10T23:03:31.7146604Z ]
2019-12-10T23:03:31.7146648Z 
2019-12-10T23:03:31.7146699Z not found errors (from test file): [
2019-12-10T23:03:31.7146747Z     Error {
2019-12-10T23:03:31.7146747Z     Error {
2019-12-10T23:03:31.7146794Z         line_num: 3,
2019-12-10T23:03:31.7146864Z         kind: Some(
2019-12-10T23:03:31.7146911Z             Error,
2019-12-10T23:03:31.7146956Z         ),
2019-12-10T23:03:31.7147023Z         msg: "binary operation `*` cannot be applied to",
2019-12-10T23:03:31.7147125Z ]
2019-12-10T23:03:31.7147155Z 
2019-12-10T23:03:31.7147554Z thread '[ui] ui/pattern/pattern-tyvar-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7147601Z 
2019-12-10T23:03:31.7147601Z 
2019-12-10T23:03:31.7147862Z ---- [ui] ui/span/issue-39018.rs stdout ----
2019-12-10T23:03:31.7147917Z 
2019-12-10T23:03:31.7148327Z error: /checkout/src/test/ui/span/issue-39018.rs:2: unexpected error: '2:22: 2:23: cannot add `&str` to `&str` [E0369]'
2019-12-10T23:03:31.7148370Z 
2019-12-10T23:03:31.7148711Z error: /checkout/src/test/ui/span/issue-39018.rs:8: unexpected error: '8:26: 8:27: cannot add `World` to `World` [E0369]'
2019-12-10T23:03:31.7148765Z 
2019-12-10T23:03:31.7149135Z error: /checkout/src/test/ui/span/issue-39018.rs:11: unexpected error: '11:22: 11:23: cannot add `std::string::String` to `&str` [E0369]'
2019-12-10T23:03:31.7149181Z 
2019-12-10T23:03:31.7149564Z error: /checkout/src/test/ui/span/issue-39018.rs:26: unexpected error: '26:16: 26:17: cannot add `&std::string::String` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7149634Z 
2019-12-10T23:03:31.7150019Z error: /checkout/src/test/ui/span/issue-39018.rs:27: unexpected error: '27:16: 27:17: cannot add `std::string::String` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7150065Z 
2019-12-10T23:03:31.7150450Z error: /checkout/src/test/ui/span/issue-39018.rs:30: unexpected error: '30:15: 30:16: cannot add `std::string::String` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7150495Z 
2019-12-10T23:03:31.7150866Z error: /checkout/src/test/ui/span/issue-39018.rs:31: unexpected error: '31:15: 31:16: cannot add `&std::string::String` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7150926Z 
2019-12-10T23:03:31.7151484Z error: /checkout/src/test/ui/span/issue-39018.rs:32: unexpected error: '32:15: 32:16: cannot add `&str` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7151547Z 
2019-12-10T23:03:31.7152064Z error: /checkout/src/test/ui/span/issue-39018.rs:33: unexpected error: '33:15: 33:16: cannot add `&&str` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7152127Z 
2019-12-10T23:03:31.7152473Z error: /checkout/src/test/ui/span/issue-39018.rs:34: unexpected error: '34:16: 34:17: cannot add `&&str` to `&&str` [E0369]'
2019-12-10T23:03:31.7152516Z 
2019-12-10T23:03:31.7152876Z error: /checkout/src/test/ui/span/issue-39018.rs:35: unexpected error: '35:16: 35:17: cannot add `&str` to `&&str` [E0369]'
2019-12-10T23:03:31.7152920Z 
2019-12-10T23:03:31.7153257Z error: /checkout/src/test/ui/span/issue-39018.rs:36: unexpected error: '36:15: 36:16: cannot add `&&str` to `&str` [E0369]'
2019-12-10T23:03:31.7153299Z 
---
2019-12-10T23:03:31.7158715Z error: /checkout/src/test/ui/span/issue-39018.rs:37: expected error not found: binary operation
2019-12-10T23:03:31.7158769Z 
2019-12-10T23:03:31.7158822Z error: 13 unexpected errors found, 13 expected errors not found
2019-12-10T23:03:31.7158882Z status: exit code: 1
2019-12-10T23:03:31.7159725Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7159897Z     Error {
2019-12-10T23:03:31.7159948Z         line_num: 2,
2019-12-10T23:03:31.7159996Z         kind: Some(
2019-12-10T23:03:31.7160059Z             Error,
2019-12-10T23:03:31.7160059Z             Error,
2019-12-10T23:03:31.7160219Z         ),
2019-12-10T23:03:31.7160282Z         msg: "2:22: 2:23: cannot add `&str` to `&str` [E0369]",
2019-12-10T23:03:31.7160464Z     Error {
2019-12-10T23:03:31.7160511Z         line_num: 8,
2019-12-10T23:03:31.7160557Z         kind: Some(
2019-12-10T23:03:31.7160618Z             Error,
2019-12-10T23:03:31.7160618Z             Error,
2019-12-10T23:03:31.7160663Z         ),
2019-12-10T23:03:31.7160716Z         msg: "8:26: 8:27: cannot add `World` to `World` [E0369]",
2019-12-10T23:03:31.7160826Z     Error {
2019-12-10T23:03:31.7160872Z         line_num: 11,
2019-12-10T23:03:31.7160919Z         kind: Some(
2019-12-10T23:03:31.7160983Z             Error,
2019-12-10T23:03:31.7160983Z             Error,
2019-12-10T23:03:31.7161028Z         ),
2019-12-10T23:03:31.7161083Z         msg: "11:22: 11:23: cannot add `std::string::String` to `&str` [E0369]",
2019-12-10T23:03:31.7161198Z     Error {
2019-12-10T23:03:31.7161244Z         line_num: 26,
2019-12-10T23:03:31.7161320Z         kind: Some(
2019-12-10T23:03:31.7161368Z             Error,
2019-12-10T23:03:31.7161368Z             Error,
2019-12-10T23:03:31.7161413Z         ),
2019-12-10T23:03:31.7161469Z         msg: "26:16: 26:17: cannot add `&std::string::String` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7161595Z     Error {
2019-12-10T23:03:31.7161640Z         line_num: 27,
2019-12-10T23:03:31.7161705Z         kind: Some(
2019-12-10T23:03:31.7161751Z             Error,
2019-12-10T23:03:31.7161751Z             Error,
2019-12-10T23:03:31.7161796Z         ),
2019-12-10T23:03:31.7161868Z         msg: "27:16: 27:17: cannot add `std::string::String` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7161965Z     Error {
2019-12-10T23:03:31.7162026Z         line_num: 30,
2019-12-10T23:03:31.7162130Z         kind: Some(
2019-12-10T23:03:31.7162175Z             Error,
2019-12-10T23:03:31.7162175Z             Error,
2019-12-10T23:03:31.7162219Z         ),
2019-12-10T23:03:31.7162287Z         msg: "30:15: 30:16: cannot add `std::string::String` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7162392Z     Error {
2019-12-10T23:03:31.7162457Z         line_num: 31,
2019-12-10T23:03:31.7162503Z         kind: Some(
2019-12-10T23:03:31.7162556Z             Error,
2019-12-10T23:03:31.7162556Z             Error,
2019-12-10T23:03:31.7162620Z         ),
2019-12-10T23:03:31.7162677Z         msg: "31:15: 31:16: cannot add `&std::string::String` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7162773Z     Error {
2019-12-10T23:03:31.7162835Z         line_num: 32,
2019-12-10T23:03:31.7162881Z         kind: Some(
2019-12-10T23:03:31.7162926Z             Error,
2019-12-10T23:03:31.7162926Z             Error,
2019-12-10T23:03:31.7162983Z         ),
2019-12-10T23:03:31.7163037Z         msg: "32:15: 32:16: cannot add `&str` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7163146Z     Error {
2019-12-10T23:03:31.7163191Z         line_num: 33,
2019-12-10T23:03:31.7163237Z         kind: Some(
2019-12-10T23:03:31.7163282Z             Error,
2019-12-10T23:03:31.7163282Z             Error,
2019-12-10T23:03:31.7163340Z         ),
2019-12-10T23:03:31.7163400Z         msg: "33:15: 33:16: cannot add `&&str` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7163518Z     Error {
2019-12-10T23:03:31.7163564Z         line_num: 34,
2019-12-10T23:03:31.7163609Z         kind: Some(
2019-12-10T23:03:31.7163654Z             Error,
2019-12-10T23:03:31.7163654Z             Error,
2019-12-10T23:03:31.7163718Z         ),
2019-12-10T23:03:31.7163770Z         msg: "34:16: 34:17: cannot add `&&str` to `&&str` [E0369]",
2019-12-10T23:03:31.7163885Z     Error {
2019-12-10T23:03:31.7163931Z         line_num: 35,
2019-12-10T23:03:31.7163975Z         kind: Some(
2019-12-10T23:03:31.7164038Z             Error,
2019-12-10T23:03:31.7164038Z             Error,
2019-12-10T23:03:31.7164083Z         ),
2019-12-10T23:03:31.7164134Z         msg: "35:16: 35:17: cannot add `&str` to `&&str` [E0369]",
2019-12-10T23:03:31.7164244Z     Error {
2019-12-10T23:03:31.7164290Z         line_num: 36,
2019-12-10T23:03:31.7164335Z         kind: Some(
2019-12-10T23:03:31.7164468Z             Error,
2019-12-10T23:03:31.7164468Z             Error,
2019-12-10T23:03:31.7164522Z         ),
2019-12-10T23:03:31.7164574Z         msg: "36:15: 36:16: cannot add `&&str` to `&str` [E0369]",
2019-12-10T23:03:31.7164746Z     Error {
2019-12-10T23:03:31.7164792Z         line_num: 37,
2019-12-10T23:03:31.7164892Z         kind: Some(
2019-12-10T23:03:31.7164956Z             Error,
2019-12-10T23:03:31.7164956Z             Error,
2019-12-10T23:03:31.7165001Z         ),
2019-12-10T23:03:31.7165052Z         msg: "37:15: 37:16: cannot add `&str` to `&str` [E0369]",
2019-12-10T23:03:31.7165373Z ]
2019-12-10T23:03:31.7165413Z 
2019-12-10T23:03:31.7165462Z not found errors (from test file): [
2019-12-10T23:03:31.7165527Z     Error {
---
2019-12-10T23:03:31.7170875Z thread '[ui] ui/span/issue-39018.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7170922Z 
2019-12-10T23:03:31.7171185Z ---- [ui] ui/str/str-concat-on-double-ref.rs stdout ----
2019-12-10T23:03:31.7171232Z 
2019-12-10T23:03:31.7171866Z error: /checkout/src/test/ui/str/str-concat-on-double-ref.rs:4: unexpected error: '4:15: 4:16: cannot add `&str` to `&std::string::String` [E0369]'
2019-12-10T23:03:31.7171927Z 
2019-12-10T23:03:31.7172351Z error: /checkout/src/test/ui/str/str-concat-on-double-ref.rs:4: expected error not found: binary operation `+` cannot be applied to type `&std::string::String`
2019-12-10T23:03:31.7172466Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7172516Z status: exit code: 1
2019-12-10T23:03:31.7172516Z status: exit code: 1
2019-12-10T23:03:31.7179322Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-concat-on-double-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-concat-on-double-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-concat-on-double-ref/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7179528Z     Error {
2019-12-10T23:03:31.7179583Z         line_num: 4,
2019-12-10T23:03:31.7179633Z         kind: Some(
2019-12-10T23:03:31.7179696Z             Error,
2019-12-10T23:03:31.7179696Z             Error,
2019-12-10T23:03:31.7179741Z         ),
2019-12-10T23:03:31.7179796Z         msg: "4:15: 4:16: cannot add `&str` to `&std::string::String` [E0369]",
2019-12-10T23:03:31.7179913Z ]
2019-12-10T23:03:31.7179944Z 
2019-12-10T23:03:31.7179992Z not found errors (from test file): [
2019-12-10T23:03:31.7180060Z     Error {
2019-12-10T23:03:31.7180060Z     Error {
2019-12-10T23:03:31.7180107Z         line_num: 4,
2019-12-10T23:03:31.7180162Z         kind: Some(
2019-12-10T23:03:31.7180208Z             Error,
2019-12-10T23:03:31.7180274Z         ),
2019-12-10T23:03:31.7180329Z         msg: "binary operation `+` cannot be applied to type `&std::string::String`",
2019-12-10T23:03:31.7180447Z ]
2019-12-10T23:03:31.7180476Z 
2019-12-10T23:03:31.7180863Z thread '[ui] ui/str/str-concat-on-double-ref.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7180909Z 
2019-12-10T23:03:31.7180909Z 
2019-12-10T23:03:31.7181223Z ---- [ui] ui/terminal-width/non-1-width-unicode-multiline-label.rs stdout ----
2019-12-10T23:03:31.7181264Z 
2019-12-10T23:03:31.7181641Z error: /checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs:5: unexpected error: '5:260: 5:261: cannot add `&str` to `&str` [E0369]'
2019-12-10T23:03:31.7181703Z 
2019-12-10T23:03:31.7182229Z error: /checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs:5: expected error not found: binary operation `+` cannot be applied to type `&str`
2019-12-10T23:03:31.7182371Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7182501Z status: exit code: 1
2019-12-10T23:03:31.7182501Z status: exit code: 1
2019-12-10T23:03:31.7184366Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7184567Z     Error {
2019-12-10T23:03:31.7185767Z         line_num: 5,
2019-12-10T23:03:31.7185845Z         kind: Some(
2019-12-10T23:03:31.7185892Z             Error,
2019-12-10T23:03:31.7185892Z             Error,
2019-12-10T23:03:31.7185961Z         ),
2019-12-10T23:03:31.7186022Z         msg: "5:260: 5:261: cannot add `&str` to `&str` [E0369]",
2019-12-10T23:03:31.7186140Z ]
2019-12-10T23:03:31.7186171Z 
2019-12-10T23:03:31.7186218Z not found errors (from test file): [
2019-12-10T23:03:31.7186267Z     Error {
2019-12-10T23:03:31.7186267Z     Error {
2019-12-10T23:03:31.7186331Z         line_num: 5,
2019-12-10T23:03:31.7186378Z         kind: Some(
2019-12-10T23:03:31.7186423Z             Error,
2019-12-10T23:03:31.7186486Z         ),
2019-12-10T23:03:31.7186539Z         msg: "binary operation `+` cannot be applied to type `&str`",
2019-12-10T23:03:31.7186630Z ]
2019-12-10T23:03:31.7186673Z 
2019-12-10T23:03:31.7187192Z thread '[ui] ui/terminal-width/non-1-width-unicode-multiline-label.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7187254Z 
2019-12-10T23:03:31.7187254Z 
2019-12-10T23:03:31.7187558Z ---- [ui] ui/traits/trait-resolution-in-overloaded-op.rs stdout ----
2019-12-10T23:03:31.7187598Z 
2019-12-10T23:03:31.7187974Z error: /checkout/src/test/ui/traits/trait-resolution-in-overloaded-op.rs:8: unexpected error: '8:7: 8:8: cannot multiply `f64` to `&T` [E0369]'
2019-12-10T23:03:31.7188082Z 
2019-12-10T23:03:31.7188460Z error: /checkout/src/test/ui/traits/trait-resolution-in-overloaded-op.rs:8: expected error not found: binary operation `*` cannot be applied to type `&T`
2019-12-10T23:03:31.7188557Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7188608Z status: exit code: 1
2019-12-10T23:03:31.7188608Z status: exit code: 1
2019-12-10T23:03:31.7189526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-resolution-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-resolution-in-overloaded-op/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7189713Z     Error {
2019-12-10T23:03:31.7189762Z         line_num: 8,
2019-12-10T23:03:31.7189827Z         kind: Some(
2019-12-10T23:03:31.7189874Z             Error,
2019-12-10T23:03:31.7189874Z             Error,
2019-12-10T23:03:31.7189919Z         ),
2019-12-10T23:03:31.7190175Z         msg: "8:7: 8:8: cannot multiply `f64` to `&T` [E0369]",
2019-12-10T23:03:31.7190297Z ]
2019-12-10T23:03:31.7190327Z 
2019-12-10T23:03:31.7190387Z not found errors (from test file): [
2019-12-10T23:03:31.7190434Z     Error {
2019-12-10T23:03:31.7190434Z     Error {
2019-12-10T23:03:31.7190480Z         line_num: 8,
2019-12-10T23:03:31.7190525Z         kind: Some(
2019-12-10T23:03:31.7190762Z             Error,
2019-12-10T23:03:31.7190820Z         ),
2019-12-10T23:03:31.7190873Z         msg: "binary operation `*` cannot be applied to type `&T`",
2019-12-10T23:03:31.7193778Z ]
2019-12-10T23:03:31.7194026Z 
2019-12-10T23:03:31.7194568Z thread '[ui] ui/traits/trait-resolution-in-overloaded-op.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7194645Z 
2019-12-10T23:03:31.7194645Z 
2019-12-10T23:03:31.7194970Z ---- [ui] ui/type/type-check/missing_trait_impl.rs stdout ----
2019-12-10T23:03:31.7195008Z 
2019-12-10T23:03:31.7195890Z error: /checkout/src/test/ui/type/type-check/missing_trait_impl.rs:5: unexpected error: '5:15: 5:16: cannot add `T` to `T` [E0369]'
2019-12-10T23:03:31.7195950Z 
2019-12-10T23:03:31.7196323Z error: /checkout/src/test/ui/type/type-check/missing_trait_impl.rs:5: expected error not found: binary operation `+` cannot be applied to type `T`
2019-12-10T23:03:31.7196454Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7196507Z status: exit code: 1
2019-12-10T23:03:31.7196507Z status: exit code: 1
2019-12-10T23:03:31.7197389Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/missing_trait_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/missing_trait_impl/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7197596Z     Error {
2019-12-10T23:03:31.7197648Z         line_num: 5,
2019-12-10T23:03:31.7197695Z         kind: Some(
2019-12-10T23:03:31.7197741Z             Error,
2019-12-10T23:03:31.7197741Z             Error,
2019-12-10T23:03:31.7197812Z         ),
2019-12-10T23:03:31.7197866Z         msg: "5:15: 5:16: cannot add `T` to `T` [E0369]",
2019-12-10T23:03:31.7197988Z ]
2019-12-10T23:03:31.7198018Z 
2019-12-10T23:03:31.7198066Z not found errors (from test file): [
2019-12-10T23:03:31.7198113Z     Error {
2019-12-10T23:03:31.7198113Z     Error {
2019-12-10T23:03:31.7198175Z         line_num: 5,
2019-12-10T23:03:31.7198221Z         kind: Some(
2019-12-10T23:03:31.7198266Z             Error,
2019-12-10T23:03:31.7198391Z         ),
2019-12-10T23:03:31.7198444Z         msg: "binary operation `+` cannot be applied to type `T`",
2019-12-10T23:03:31.7198536Z ]
2019-12-10T23:03:31.7198585Z 
2019-12-10T23:03:31.7198950Z thread '[ui] ui/type/type-check/missing_trait_impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7198997Z 
2019-12-10T23:03:31.7198997Z 
2019-12-10T23:03:31.7199270Z ---- [ui] ui/vec/vec-res-add.rs stdout ----
2019-12-10T23:03:31.7199307Z 
2019-12-10T23:03:31.7199682Z error: /checkout/src/test/ui/vec/vec-res-add.rs:16: unexpected error: '16:15: 16:16: cannot add `std::vec::Vec<R>` to `std::vec::Vec<R>` [E0369]'
2019-12-10T23:03:31.7199734Z 
2019-12-10T23:03:31.7203311Z error: /checkout/src/test/ui/vec/vec-res-add.rs:16: expected error not found: binary operation `+` cannot be applied to type
2019-12-10T23:03:31.7203430Z error: 1 unexpected errors found, 1 expected errors not found
2019-12-10T23:03:31.7203481Z status: exit code: 1
2019-12-10T23:03:31.7203481Z status: exit code: 1
2019-12-10T23:03:31.7204488Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/vec/vec-res-add.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-res-add/auxiliary" "-A" "unused"
2019-12-10T23:03:31.7204680Z     Error {
2019-12-10T23:03:31.7204813Z         line_num: 16,
2019-12-10T23:03:31.7204877Z         kind: Some(
2019-12-10T23:03:31.7205233Z             Error,
2019-12-10T23:03:31.7205233Z             Error,
2019-12-10T23:03:31.7205281Z         ),
2019-12-10T23:03:31.7205336Z         msg: "16:15: 16:16: cannot add `std::vec::Vec<R>` to `std::vec::Vec<R>` [E0369]",
2019-12-10T23:03:31.7205452Z ]
2019-12-10T23:03:31.7205482Z 
2019-12-10T23:03:31.7205547Z not found errors (from test file): [
2019-12-10T23:03:31.7206388Z     Error {
2019-12-10T23:03:31.7206388Z     Error {
2019-12-10T23:03:31.7206436Z         line_num: 16,
2019-12-10T23:03:31.7206483Z         kind: Some(
2019-12-10T23:03:31.7206553Z             Error,
2019-12-10T23:03:31.7206598Z         ),
2019-12-10T23:03:31.7206649Z         msg: "binary operation `+` cannot be applied to type",
2019-12-10T23:03:31.7206759Z ]
2019-12-10T23:03:31.7206788Z 
2019-12-10T23:03:31.7207402Z thread '[ui] ui/vec/vec-res-add.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1534:13
2019-12-10T23:03:31.7207480Z 
---
2019-12-10T23:03:31.7219317Z test result: FAILED. 9273 passed; 22 failed; 47 ignored; 0 measured; 0 filtered out
2019-12-10T23:03:31.7219358Z 
2019-12-10T23:03:31.7219399Z 
2019-12-10T23:03:31.7219428Z 
2019-12-10T23:03:31.7222096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-10T23:03:31.7222447Z 
2019-12-10T23:03:31.7222499Z 
2019-12-10T23:03:31.7222552Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-10T23:03:31.7222608Z Build completed unsuccessfully in 1:02:27
2019-12-10T23:03:31.7222608Z Build completed unsuccessfully in 1:02:27
2019-12-10T23:03:31.7222677Z == clock drift check ==
2019-12-10T23:03:31.7222728Z   local time: Tue Dec 10 23:03:31 UTC 2019
2019-12-10T23:03:32.0064476Z   network time: Tue, 10 Dec 2019 23:03:32 GMT
2019-12-10T23:03:32.0070458Z == end clock drift check ==
2019-12-10T23:03:32.9524290Z 
2019-12-10T23:03:32.9653120Z ##[error]Bash exited with code '1'.
2019-12-10T23:03:32.9702158Z ##[section]Starting: Checkout
2019-12-10T23:03:32.9704619Z ==============================================================================
2019-12-10T23:03:32.9704705Z Task         : Get sources
2019-12-10T23:03:32.9704758Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

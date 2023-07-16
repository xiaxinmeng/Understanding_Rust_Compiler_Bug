plain
2019-10-19T06:33:14.4827252Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T06:33:14.5042040Z ##[command]git config gc.auto 0
2019-10-19T06:33:14.5139531Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T06:33:14.5219455Z ##[command]git config --get-all http.proxy
2019-10-19T06:33:14.5380780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65112/merge:refs/remotes/pull/65112/merge
---
2019-10-19T07:39:57.7825949Z .................................................................................................... 1600/9198
2019-10-19T07:40:03.2626628Z .................................................................................................... 1700/9198
2019-10-19T07:40:17.0204126Z .............................i...............i...................................................... 1800/9198
2019-10-19T07:40:24.8703153Z .................................................................................................... 1900/9198
2019-10-19T07:40:40.2707045Z ...................iiiii............................................................................ 2000/9198
2019-10-19T07:40:51.3545086Z .................................................................................................... 2200/9198
2019-10-19T07:40:54.0320530Z .................................................................................................... 2300/9198
2019-10-19T07:40:59.7236790Z .................................................................................................... 2400/9198
2019-10-19T07:41:23.4485631Z .................................................................................................... 2500/9198
---
2019-10-19T07:44:29.8943044Z ......................i...............i............................................................. 4800/9198
2019-10-19T07:44:42.3474707Z .................................................................................................... 4900/9198
2019-10-19T07:44:49.0280857Z .................................................................................................... 5000/9198
2019-10-19T07:44:59.9428739Z .................................................................................................... 5100/9198
2019-10-19T07:45:06.7080061Z ......................ii.ii......................................................................... 5200/9198
2019-10-19T07:45:17.8183097Z .................................................................................................... 5400/9198
2019-10-19T07:45:28.4671902Z ........................................................................................i........... 5500/9198
2019-10-19T07:45:37.0476589Z .................................................................................................... 5600/9198
2019-10-19T07:45:42.4645699Z .................................................................................................... 5700/9198
2019-10-19T07:45:42.4645699Z .................................................................................................... 5700/9198
2019-10-19T07:45:53.6904680Z .....................................................................................ii...i..ii..... 5800/9198
2019-10-19T07:46:21.3460390Z .................................................................................................... 6000/9198
2019-10-19T07:46:31.3916846Z .................................................................................................... 6100/9198
2019-10-19T07:46:41.0209742Z .................................................................................................... 6200/9198
2019-10-19T07:46:41.0209742Z .................................................................................................... 6200/9198
2019-10-19T07:46:57.7030744Z .......i..ii........................................................................................ 6300/9198
2019-10-19T07:47:18.6130875Z ...................................................................i................................ 6500/9198
2019-10-19T07:47:20.9596619Z .................................................................................................... 6600/9198
2019-10-19T07:47:23.6118289Z ..........................................i......................................................... 6700/9198
2019-10-19T07:47:27.7315729Z .................................................................................................... 6800/9198
---
2019-10-19T07:51:48.8238870Z normalized stderr:
2019-10-19T07:51:48.8239081Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8239484Z   --> $DIR/alignment-gep-tup-like-1.rs:31:10
2019-10-19T07:51:48.8240020Z    |
2019-10-19T07:51:48.8240735Z LL |     } as (Box<dyn Invokable<A>+'static>)
2019-10-19T07:51:48.8241052Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8241398Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8241535Z 
2019-10-19T07:51:48.8241666Z 
2019-10-19T07:51:48.8241817Z 
2019-10-19T07:51:48.8241817Z 
2019-10-19T07:51:48.8241948Z 
2019-10-19T07:51:48.8242104Z The actual stderr differed from the expected stderr.
2019-10-19T07:51:48.8242603Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/alignment-gep-tup-like-1.stderr
2019-10-19T07:51:48.8243065Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T07:51:48.8243536Z To only update this specific test, also pass `--test-args alignment-gep-tup-like-1.rs`
2019-10-19T07:51:48.8243897Z error: 1 errors occurred comparing output.
2019-10-19T07:51:48.8244051Z status: exit code: 0
2019-10-19T07:51:48.8244051Z status: exit code: 0
2019-10-19T07:51:48.8245041Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/alignment-gep-tup-like-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alignment-gep-tup-like-1/auxiliary"
2019-10-19T07:51:48.8246462Z ------------------------------------------
2019-10-19T07:51:48.8246907Z 
2019-10-19T07:51:48.8247450Z ------------------------------------------
2019-10-19T07:51:48.8247683Z stderr:
2019-10-19T07:51:48.8247683Z stderr:
2019-10-19T07:51:48.8248072Z ------------------------------------------
2019-10-19T07:51:48.8248317Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8248725Z   --> /checkout/src/test/ui/alignment-gep-tup-like-1.rs:31:10
2019-10-19T07:51:48.8249093Z    |
2019-10-19T07:51:48.8249950Z LL |     } as (Box<dyn Invokable<A>+'static>)
2019-10-19T07:51:48.8250177Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8250506Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8250638Z 
2019-10-19T07:51:48.8250767Z 
2019-10-19T07:51:48.8251181Z ------------------------------------------
2019-10-19T07:51:48.8251181Z ------------------------------------------
2019-10-19T07:51:48.8251380Z 
2019-10-19T07:51:48.8251520Z 
2019-10-19T07:51:48.8251880Z ---- [ui] ui/as-precedence.rs stdout ----
2019-10-19T07:51:48.8252075Z normalized stderr:
2019-10-19T07:51:48.8252282Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8252879Z   --> $DIR/as-precedence.rs:5:21
2019-10-19T07:51:48.8253102Z    |
2019-10-19T07:51:48.8253259Z LL |     assert_eq!(3 as (usize) * 3, 9);
2019-10-19T07:51:48.8253413Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8253736Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8253871Z 
2019-10-19T07:51:48.8254043Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8254404Z   --> $DIR/as-precedence.rs:6:21
2019-10-19T07:51:48.8254404Z   --> $DIR/as-precedence.rs:6:21
2019-10-19T07:51:48.8254612Z    |
2019-10-19T07:51:48.8254763Z LL |     assert_eq!(3 as (usize) / 3, 1);
2019-10-19T07:51:48.8255418Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8255746Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8256204Z   --> $DIR/as-precedence.rs:8:21
2019-10-19T07:51:48.8256415Z    |
2019-10-19T07:51:48.8256415Z    |
2019-10-19T07:51:48.8256616Z LL |     assert_eq!(3 as (usize) + 3, 6);
2019-10-19T07:51:48.8256783Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8257124Z 
2019-10-19T07:51:48.8257299Z 
2019-10-19T07:51:48.8257454Z 
2019-10-19T07:51:48.8257617Z The actual stderr differed from the expected stderr.
2019-10-19T07:51:48.8257617Z The actual stderr differed from the expected stderr.
2019-10-19T07:51:48.8258098Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/as-precedence/as-precedence.stderr
2019-10-19T07:51:48.8258568Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T07:51:48.8259164Z To only update this specific test, also pass `--test-args as-precedence.rs`
2019-10-19T07:51:48.8259544Z error: 1 errors occurred comparing output.
2019-10-19T07:51:48.8259693Z status: exit code: 0
2019-10-19T07:51:48.8259693Z status: exit code: 0
2019-10-19T07:51:48.8260497Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/as-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/as-precedence/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/as-precedence/auxiliary"
2019-10-19T07:51:48.8261148Z ------------------------------------------
2019-10-19T07:51:48.8261327Z 
2019-10-19T07:51:48.8261678Z ------------------------------------------
2019-10-19T07:51:48.8262084Z stderr:
2019-10-19T07:51:48.8262084Z stderr:
2019-10-19T07:51:48.8262643Z ------------------------------------------
2019-10-19T07:51:48.8262854Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8263247Z   --> /checkout/src/test/ui/as-precedence.rs:5:21
2019-10-19T07:51:48.8263451Z    |
2019-10-19T07:51:48.8263792Z LL |     assert_eq!(3 as (usize) * 3, 9);
2019-10-19T07:51:48.8263954Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8264643Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8265299Z 
2019-10-19T07:51:48.8265501Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8266225Z   --> /checkout/src/test/ui/as-precedence.rs:6:21
2019-10-19T07:51:48.8266225Z   --> /checkout/src/test/ui/as-precedence.rs:6:21
2019-10-19T07:51:48.8269281Z    |
2019-10-19T07:51:48.8269561Z LL |     assert_eq!(3 as (usize) / 3, 1);
2019-10-19T07:51:48.8269723Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8270194Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8271372Z   --> /checkout/src/test/ui/as-precedence.rs:8:21
2019-10-19T07:51:48.8271601Z    |
2019-10-19T07:51:48.8271601Z    |
2019-10-19T07:51:48.8271921Z LL |     assert_eq!(3 as (usize) + 3, 6);
2019-10-19T07:51:48.8272094Z    |                     ^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8272393Z 
2019-10-19T07:51:48.8272803Z ------------------------------------------
2019-10-19T07:51:48.8273008Z 
2019-10-19T07:51:48.8273160Z 
2019-10-19T07:51:48.8273160Z 
2019-10-19T07:51:48.8273576Z ---- [ui] ui/close-over-big-then-small-data.rs stdout ----
2019-10-19T07:51:48.8274213Z normalized stderr:
2019-10-19T07:51:48.8274400Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8274824Z   --> $DIR/close-over-big-then-small-data.rs:33:10
2019-10-19T07:51:48.8275028Z    |
2019-10-19T07:51:48.8276064Z LL |     } as (Box<dyn Invokable<A>+'static>)
2019-10-19T07:51:48.8276325Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8276665Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8276823Z 
2019-10-19T07:51:48.8276961Z 
2019-10-19T07:51:48.8277101Z 
2019-10-19T07:51:48.8277101Z 
2019-10-19T07:51:48.8277239Z 
2019-10-19T07:51:48.8277420Z The actual stderr differed from the expected stderr.
2019-10-19T07:51:48.8277932Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/close-over-big-then-small-data/close-over-big-then-small-data.stderr
2019-10-19T07:51:48.8278438Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T07:51:48.8279174Z To only update this specific test, also pass `--test-args close-over-big-then-small-data.rs`
2019-10-19T07:51:48.8279549Z error: 1 errors occurred comparing output.
2019-10-19T07:51:48.8279706Z status: exit code: 0
2019-10-19T07:51:48.8279706Z status: exit code: 0
2019-10-19T07:51:48.8280596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/close-over-big-then-small-data.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/close-over-big-then-small-data/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/close-over-big-then-small-data/auxiliary"
2019-10-19T07:51:48.8281272Z ------------------------------------------
2019-10-19T07:51:48.8281461Z 
2019-10-19T07:51:48.8281835Z ------------------------------------------
2019-10-19T07:51:48.8282029Z stderr:
2019-10-19T07:51:48.8282029Z stderr:
2019-10-19T07:51:48.8282421Z ------------------------------------------
2019-10-19T07:51:48.8282647Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8283047Z   --> /checkout/src/test/ui/close-over-big-then-small-data.rs:33:10
2019-10-19T07:51:48.8283245Z    |
2019-10-19T07:51:48.8291052Z LL |     } as (Box<dyn Invokable<A>+'static>)
2019-10-19T07:51:48.8292758Z    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8293160Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8293446Z 
2019-10-19T07:51:48.8293762Z 
2019-10-19T07:51:48.8294372Z ------------------------------------------
2019-10-19T07:51:48.8294372Z ------------------------------------------
2019-10-19T07:51:48.8294551Z 
2019-10-19T07:51:48.8294694Z 
2019-10-19T07:51:48.8295809Z ---- [ui] ui/functions-closures/closure-to-fn-coercion.rs stdout ----
2019-10-19T07:51:48.8296072Z normalized stderr:
2019-10-19T07:51:48.8296256Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8296698Z   --> $DIR/closure-to-fn-coercion.rs:13:23
2019-10-19T07:51:48.8296895Z    |
2019-10-19T07:51:48.8297268Z LL | fn func_specific() -> (fn() -> u32) {
2019-10-19T07:51:48.8297487Z    |                       ^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8297818Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8297959Z 
2019-10-19T07:51:48.8298091Z 
2019-10-19T07:51:48.8298224Z 
2019-10-19T07:51:48.8298224Z 
2019-10-19T07:51:48.8298355Z 
2019-10-19T07:51:48.8298532Z The actual stderr differed from the expected stderr.
2019-10-19T07:51:48.8299168Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/closure-to-fn-coercion/closure-to-fn-coercion.stderr
2019-10-19T07:51:48.8300361Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T07:51:48.8301716Z To only update this specific test, also pass `--test-args functions-closures/closure-to-fn-coercion.rs`
2019-10-19T07:51:48.8301990Z error: 1 errors occurred comparing output.
2019-10-19T07:51:48.8302037Z status: exit code: 0
2019-10-19T07:51:48.8302037Z status: exit code: 0
2019-10-19T07:51:48.8302855Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functions-closures/closure-to-fn-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/closure-to-fn-coercion/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/closure-to-fn-coercion/auxiliary"
2019-10-19T07:51:48.8303219Z ------------------------------------------
2019-10-19T07:51:48.8303256Z 
2019-10-19T07:51:48.8303496Z ------------------------------------------
2019-10-19T07:51:48.8303553Z stderr:
2019-10-19T07:51:48.8303553Z stderr:
2019-10-19T07:51:48.8304278Z ------------------------------------------
2019-10-19T07:51:48.8304341Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8305496Z   --> /checkout/src/test/ui/functions-closures/closure-to-fn-coercion.rs:13:23
2019-10-19T07:51:48.8305584Z    |
2019-10-19T07:51:48.8305865Z LL | fn func_specific() -> (fn() -> u32) {
2019-10-19T07:51:48.8305921Z    |                       ^^^^^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8306033Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8306066Z 
2019-10-19T07:51:48.8306092Z 
2019-10-19T07:51:48.8306332Z ------------------------------------------
2019-10-19T07:51:48.8306332Z ------------------------------------------
2019-10-19T07:51:48.8306365Z 
2019-10-19T07:51:48.8306391Z 
2019-10-19T07:51:48.8306628Z ---- [ui] ui/single-use-lifetime/zero-uses-in-fn.rs stdout ----
2019-10-19T07:51:48.8306677Z 
2019-10-19T07:51:48.8306743Z error: fixed code is still producing diagnostics
2019-10-19T07:51:48.8306797Z status: exit code: 0
2019-10-19T07:51:48.8307549Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/zero-uses-in-fn/auxiliary"
2019-10-19T07:51:48.8307887Z ------------------------------------------
2019-10-19T07:51:48.8307941Z 
2019-10-19T07:51:48.8308164Z ------------------------------------------
2019-10-19T07:51:48.8308211Z stderr:
2019-10-19T07:51:48.8308211Z stderr:
2019-10-19T07:51:48.8308759Z ------------------------------------------
2019-10-19T07:51:48.8308828Z warning: unnecessary parentheses around type
2019-10-19T07:51:48.8309104Z   --> /checkout/src/test/ui/single-use-lifetime/zero-uses-in-fn.fixed:18:32
2019-10-19T07:51:48.8309172Z    |
2019-10-19T07:51:48.8309384Z LL | fn november<'a>(s: &'a str) -> (&'a str) {
2019-10-19T07:51:48.8309436Z    |                                ^^^^^^^^^ help: remove these parentheses
2019-10-19T07:51:48.8309538Z    = note: `#[warn(unused_parens)]` on by default
2019-10-19T07:51:48.8309567Z 
2019-10-19T07:51:48.8309591Z 
2019-10-19T07:51:48.8310870Z ------------------------------------------
---
2019-10-19T07:51:48.8313312Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-19T07:51:48.8313370Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-19T07:51:48.8313556Z 
2019-10-19T07:51:48.8313597Z 
2019-10-19T07:51:48.8315348Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T07:51:48.8315633Z 
2019-10-19T07:51:48.8315664Z 
2019-10-19T07:51:48.8321080Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T07:51:48.8321191Z Build completed unsuccessfully in 1:11:38
2019-10-19T07:51:48.8321191Z Build completed unsuccessfully in 1:11:38
2019-10-19T07:51:48.8372931Z == clock drift check ==
2019-10-19T07:51:48.8392291Z   local time: Sat Oct 19 07:51:48 UTC 2019
2019-10-19T07:51:49.1300268Z   network time: Sat, 19 Oct 2019 07:51:49 GMT
2019-10-19T07:51:49.1303017Z == end clock drift check ==
2019-10-19T07:51:50.3195806Z 
2019-10-19T07:51:50.3308592Z ##[error]Bash exited with code '1'.
2019-10-19T07:51:50.3349755Z ##[section]Starting: Checkout
2019-10-19T07:51:50.3352423Z ==============================================================================
2019-10-19T07:51:50.3352478Z Task         : Get sources
2019-10-19T07:51:50.3352524Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

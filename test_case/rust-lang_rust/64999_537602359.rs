plain
2019-10-02T16:33:43.0236797Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-02T16:33:43.0431505Z ##[command]git config gc.auto 0
2019-10-02T16:33:43.0484444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-02T16:33:43.0536619Z ##[command]git config --get-all http.proxy
2019-10-02T16:33:43.0655444Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64999/merge:refs/remotes/pull/64999/merge
---
2019-10-02T17:29:57.1898083Z .................................................................................................... 1400/9088
2019-10-02T17:30:03.1534227Z .................................................................................................... 1500/9088
2019-10-02T17:30:09.2989833Z .................................................................................................... 1600/9088
2019-10-02T17:30:18.0746076Z .................................................................................................... 1700/9088
2019-10-02T17:30:25.7939669Z .i...............i.................................................................................. 1800/9088
2019-10-02T17:30:32.1607494Z ............................................................................................iiiii... 1900/9088
2019-10-02T17:30:52.5110038Z .................................................................................................... 2100/9088
2019-10-02T17:30:54.7246833Z .................................................................................................... 2200/9088
2019-10-02T17:30:57.0214495Z .................................................................................................... 2300/9088
2019-10-02T17:31:02.9851870Z .................................................................................................... 2400/9088
---
2019-10-02T17:33:45.6690942Z ...............................................................................i...............i.... 4700/9088
2019-10-02T17:33:52.9412246Z .................................................................................................... 4800/9088
2019-10-02T17:34:02.1708505Z .................................................................................................... 4900/9088
2019-10-02T17:34:07.6311100Z .................................................................................................... 5000/9088
2019-10-02T17:34:17.8871593Z .......................................................................ii.ii........................ 5100/9088
2019-10-02T17:34:26.6030177Z .................................................................................................... 5300/9088
2019-10-02T17:34:35.2712063Z .................................................................................................... 5400/9088
2019-10-02T17:34:41.9173529Z .....................................i.............................................................. 5500/9088
2019-10-02T17:34:47.5921710Z .................................................................................................... 5600/9088
2019-10-02T17:34:47.5921710Z .................................................................................................... 5600/9088
2019-10-02T17:34:58.5090367Z .................................................................................................... 5700/9088
2019-10-02T17:35:08.9401299Z .................................ii...i..ii...........i............................................. 5800/9088
2019-10-02T17:35:28.9868765Z .................................................................................................... 6000/9088
2019-10-02T17:35:35.5099463Z .................................................................................................... 6100/9088
2019-10-02T17:35:35.5099463Z .................................................................................................... 6100/9088
2019-10-02T17:35:48.0583033Z ....................................i..ii........................................................... 6200/9088
2019-10-02T17:36:06.7854491Z ................................................................................................i... 6400/9088
2019-10-02T17:36:08.7167632Z .................................................................................................... 6500/9088
2019-10-02T17:36:10.6944076Z ....................................................................i............................... 6600/9088
2019-10-02T17:36:13.3432000Z .................................................................................................... 6700/9088
---
2019-10-02T17:37:03.4030150Z .................................................................................................... 7200/9088
2019-10-02T17:37:08.1380823Z .................................................................................................... 7300/9088
2019-10-02T17:37:13.3411122Z .................................................................................................... 7400/9088
2019-10-02T17:37:22.9044909Z .................................................................................................... 7500/9088
2019-10-02T17:37:30.9017971Z .............F....................F.............F.F.F.F............................................. 7600/9088
2019-10-02T17:37:38.6260829Z ........................ii......i................................................................... 7700/9088
2019-10-02T17:37:55.6998972Z .................................................................................................... 7900/9088
2019-10-02T17:38:06.6514950Z .................................................................................................... 8000/9088
2019-10-02T17:38:13.7714004Z .................................................................................................... 8100/9088
2019-10-02T17:38:50.0609169Z .................................................................................................... 8200/9088
---
2019-10-02T17:39:57.9402733Z 41 
2019-10-02T17:39:57.9402771Z + error[E0308]: mismatched types
2019-10-02T17:39:57.9403518Z +   --> $DIR/async-block-control-flow-static-semantics.rs:22:58
2019-10-02T17:39:57.9403576Z +    |
2019-10-02T17:39:57.9404181Z + LL |   async fn return_targets_async_block_not_async_fn() -> u8 {
2019-10-02T17:39:57.9404303Z + LL | |
2019-10-02T17:39:57.9404303Z + LL | |
2019-10-02T17:39:57.9404339Z + LL | |     let block = async {
2019-10-02T17:39:57.9404394Z + LL | |         return 0u8;
2019-10-02T17:39:57.9404429Z + ...  |
2019-10-02T17:39:57.9404464Z + LL | |
2019-10-02T17:39:57.9404703Z + LL | | }
2019-10-02T17:39:57.9404763Z +    | |_^ expected u8, found ()
2019-10-02T17:39:57.9404836Z +    = note: expected type `u8`
2019-10-02T17:39:57.9404892Z +               found type `()`
2019-10-02T17:39:57.9405108Z + 
2019-10-02T17:39:57.9405108Z + 
2019-10-02T17:39:57.9405152Z 42 error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-10-02T17:39:57.9405675Z 44    |
2019-10-02T17:39:57.9405697Z 
2019-10-02T17:39:57.9405731Z 48    = note: expected type `u8`
2019-10-02T17:39:57.9405795Z 49               found type `()`
2019-10-02T17:39:57.9405795Z 49               found type `()`
2019-10-02T17:39:57.9405839Z 50    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-10-02T17:39:57.9406889Z - 
2019-10-02T17:39:57.9407519Z - error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == u8`
2019-10-02T17:39:57.9408247Z -    |
2019-10-02T17:39:57.9408247Z -    |
2019-10-02T17:39:57.9408519Z - LL | async fn return_targets_async_block_not_async_fn() -> u8 {
2019-10-02T17:39:57.9409050Z -    |                                                       ^^ expected (), found u8
2019-10-02T17:39:57.9442575Z -    = note: expected type `()`
2019-10-02T17:39:57.9442818Z -               found type `u8`
2019-10-02T17:39:57.9442818Z -               found type `u8`
2019-10-02T17:39:57.9443188Z -    = note: the return type of a function must have a statically known size
2019-10-02T17:39:57.9443302Z 62 error[E0308]: mismatched types
2019-10-02T17:39:57.9443505Z 63   --> $DIR/async-block-control-flow-static-semantics.rs:48:44
2019-10-02T17:39:57.9443536Z 
2019-10-02T17:39:57.9443559Z 
2019-10-02T17:39:57.9443559Z 
2019-10-02T17:39:57.9443617Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9443917Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
2019-10-02T17:39:57.9444306Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9444573Z To only update this specific test, also pass `--test-args async-await/async-block-control-flow-static-semantics.rs`
2019-10-02T17:39:57.9444809Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9445026Z status: exit code: 1
2019-10-02T17:39:57.9445026Z status: exit code: 1
2019-10-02T17:39:57.9446187Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9446960Z ------------------------------------------
2019-10-02T17:39:57.9447013Z 
2019-10-02T17:39:57.9447269Z ------------------------------------------
2019-10-02T17:39:57.9447318Z stderr:
2019-10-02T17:39:57.9447318Z stderr:
2019-10-02T17:39:57.9447532Z ------------------------------------------
2019-10-02T17:39:57.9447603Z error[E0267]: `break` inside of an `async` block
2019-10-02T17:39:57.9447939Z    |
2019-10-02T17:39:57.9448005Z LL |       async {
2019-10-02T17:39:57.9448207Z    |  ___________-
2019-10-02T17:39:57.9448207Z    |  ___________-
2019-10-02T17:39:57.9448262Z LL | |         break 0u8; //~ ERROR `break` inside of an `async` block
2019-10-02T17:39:57.9448335Z    | |         ^^^^^^^^^ cannot `break` inside of an `async` block
2019-10-02T17:39:57.9448384Z LL | |     };
2019-10-02T17:39:57.9448597Z    | |_____- enclosing `async` block
2019-10-02T17:39:57.9448631Z 
2019-10-02T17:39:57.9455128Z error[E0267]: `break` inside of an `async` block
2019-10-02T17:39:57.9455565Z    |
2019-10-02T17:39:57.9455622Z LL |           async {
2019-10-02T17:39:57.9455788Z    |  _______________-
2019-10-02T17:39:57.9455788Z    |  _______________-
2019-10-02T17:39:57.9455831Z LL | |             break 0u8; //~ ERROR `break` inside of an `async` block
2019-10-02T17:39:57.9455898Z    | |             ^^^^^^^^^ cannot `break` inside of an `async` block
2019-10-02T17:39:57.9455938Z LL | |         };
2019-10-02T17:39:57.9456653Z    | |_________- enclosing `async` block
2019-10-02T17:39:57.9456771Z error[E0308]: mismatched types
2019-10-02T17:39:57.9457094Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:13:43
2019-10-02T17:39:57.9457147Z    |
2019-10-02T17:39:57.9457147Z    |
2019-10-02T17:39:57.9457394Z LL | fn return_targets_async_block_not_fn() -> u8 {
2019-10-02T17:39:57.9457649Z    |    ---------------------------------      ^^ expected u8, found ()
2019-10-02T17:39:57.9457701Z    |    |
2019-10-02T17:39:57.9457772Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-10-02T17:39:57.9457888Z    = note: expected type `u8`
2019-10-02T17:39:57.9457936Z               found type `()`
2019-10-02T17:39:57.9457985Z 
2019-10-02T17:39:57.9457985Z 
2019-10-02T17:39:57.9458041Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-10-02T17:39:57.9458451Z    |
2019-10-02T17:39:57.9458451Z    |
2019-10-02T17:39:57.9458502Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-10-02T17:39:57.9458558Z    |                                       ^^^^^^ expected u8, found ()
2019-10-02T17:39:57.9458674Z    = note: expected type `u8`
2019-10-02T17:39:57.9458723Z               found type `()`
2019-10-02T17:39:57.9458779Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-10-02T17:39:57.9458835Z 
2019-10-02T17:39:57.9458835Z 
2019-10-02T17:39:57.9459064Z error[E0308]: mismatched types
2019-10-02T17:39:57.9459402Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:22:58
2019-10-02T17:39:57.9459478Z    |
2019-10-02T17:39:57.9459745Z LL |   async fn return_targets_async_block_not_async_fn() -> u8 {
2019-10-02T17:39:57.9460055Z    |  __________________________________________________________^
2019-10-02T17:39:57.9460297Z LL | |     //~^ ERROR type mismatch resolving
2019-10-02T17:39:57.9460334Z LL | |     let block = async {
2019-10-02T17:39:57.9460370Z LL | |         return 0u8;
2019-10-02T17:39:57.9460405Z ...  |
2019-10-02T17:39:57.9460465Z LL | |     //~^ ERROR type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-10-02T17:39:57.9460505Z LL | | }
2019-10-02T17:39:57.9460540Z    | |_^ expected u8, found ()
2019-10-02T17:39:57.9460629Z    = note: expected type `u8`
2019-10-02T17:39:57.9460664Z               found type `()`
2019-10-02T17:39:57.9460694Z 
2019-10-02T17:39:57.9460694Z 
2019-10-02T17:39:57.9460752Z error[E0271]: type mismatch resolving `<impl std::future::Future as std::future::Future>::Output == ()`
2019-10-02T17:39:57.9461042Z    |
2019-10-02T17:39:57.9461042Z    |
2019-10-02T17:39:57.9461104Z LL |     let _: &dyn Future<Output = ()> = &block;
2019-10-02T17:39:57.9461147Z    |                                       ^^^^^^ expected u8, found ()
2019-10-02T17:39:57.9461233Z    = note: expected type `u8`
2019-10-02T17:39:57.9461268Z               found type `()`
2019-10-02T17:39:57.9461311Z    = note: required for the cast to the object type `dyn std::future::Future<Output = ()>`
2019-10-02T17:39:57.9461337Z 
2019-10-02T17:39:57.9461337Z 
2019-10-02T17:39:57.9461391Z error[E0308]: mismatched types
2019-10-02T17:39:57.9461606Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:48:44
2019-10-02T17:39:57.9461646Z    |
2019-10-02T17:39:57.9461857Z LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
2019-10-02T17:39:57.9462088Z    |    ----------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-10-02T17:39:57.9462130Z    |    |
2019-10-02T17:39:57.9462188Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-10-02T17:39:57.9462233Z    |
2019-10-02T17:39:57.9462273Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-10-02T17:39:57.9462355Z 
2019-10-02T17:39:57.9462391Z error[E0308]: mismatched types
2019-10-02T17:39:57.9462622Z   --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:57:50
2019-10-02T17:39:57.9462683Z    |
2019-10-02T17:39:57.9462683Z    |
2019-10-02T17:39:57.9462890Z LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
2019-10-02T17:39:57.9463135Z    |    ----------------------------------------      ^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-10-02T17:39:57.9463208Z    |    |
2019-10-02T17:39:57.9463253Z    |    implicitly returns `()` as its body has no tail or `return` expression
2019-10-02T17:39:57.9463291Z    |
2019-10-02T17:39:57.9463346Z    = note: expected type `std::result::Result<u8, MyErr>`
2019-10-02T17:39:57.9463411Z 
2019-10-02T17:39:57.9463454Z error: aborting due to 8 previous errors
2019-10-02T17:39:57.9463498Z 
2019-10-02T17:39:57.9463536Z Some errors have detailed explanations: E0267, E0271, E0308.
---
2019-10-02T17:39:57.9464036Z 
2019-10-02T17:39:57.9464230Z ---- [ui] ui/async-await/async-error-span.rs stdout ----
2019-10-02T17:39:57.9464289Z diff of stderr:
2019-10-02T17:39:57.9464313Z 
2019-10-02T17:39:57.9464520Z - error[E0698]: type inside `async` object must be known in this context
2019-10-02T17:39:57.9464673Z + error[E0698]: type inside `async` fn body must be known in this context
2019-10-02T17:39:57.9471325Z 3    |
2019-10-02T17:39:57.9471411Z 4 LL |     let a;
2019-10-02T17:39:57.9471458Z 
2019-10-02T17:39:57.9471644Z 5    |         ^ cannot infer type
2019-10-02T17:39:57.9471644Z 5    |         ^ cannot infer type
2019-10-02T17:39:57.9471692Z 6    |
2019-10-02T17:39:57.9472067Z - note: the type is part of the `async` object because of this `await`
2019-10-02T17:39:57.9472116Z + note: the type is part of the `async` fn body because of this `await`
2019-10-02T17:39:57.9472360Z 9    |
2019-10-02T17:39:57.9472360Z 9    |
2019-10-02T17:39:57.9472622Z 10 LL |     get_future().await;
2019-10-02T17:39:57.9472667Z 
2019-10-02T17:39:57.9472705Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9479180Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
2019-10-02T17:39:57.9479180Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
2019-10-02T17:39:57.9479531Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9479812Z To only update this specific test, also pass `--test-args async-await/async-error-span.rs`
2019-10-02T17:39:57.9479950Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9480002Z status: exit code: 1
2019-10-02T17:39:57.9480002Z status: exit code: 1
2019-10-02T17:39:57.9481389Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-error-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9481849Z ------------------------------------------
2019-10-02T17:39:57.9481878Z 
2019-10-02T17:39:57.9482064Z ------------------------------------------
2019-10-02T17:39:57.9482123Z stderr:
2019-10-02T17:39:57.9482123Z stderr:
2019-10-02T17:39:57.9482304Z ------------------------------------------
2019-10-02T17:39:57.9482357Z error[E0698]: type inside `async` fn body must be known in this context
2019-10-02T17:39:57.9482786Z    |
2019-10-02T17:39:57.9482786Z    |
2019-10-02T17:39:57.9482827Z LL |     let a; //~ ERROR type inside `async` object must be known in this context
2019-10-02T17:39:57.9482923Z    |
2019-10-02T17:39:57.9482923Z    |
2019-10-02T17:39:57.9482962Z note: the type is part of the `async` fn body because of this `await`
2019-10-02T17:39:57.9483406Z    |
2019-10-02T17:39:57.9483406Z    |
2019-10-02T17:39:57.9483449Z LL |     get_future().await;
2019-10-02T17:39:57.9483529Z 
2019-10-02T17:39:57.9483568Z error: aborting due to previous error
2019-10-02T17:39:57.9483593Z 
2019-10-02T17:39:57.9483957Z For more information about this error, try `rustc --explain E0698`.
---
2019-10-02T17:39:57.9484545Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9484744Z -   --> $DIR/issue-63388-1.rs:12:10
2019-10-02T17:39:57.9484923Z +   --> $DIR/issue-63388-1.rs:14:9
2019-10-02T17:39:57.9484962Z 3    |
2019-10-02T17:39:57.9485132Z 4 LL |         &'a self, foo: &dyn Foo
2019-10-02T17:39:57.9485371Z 5    |         -------- this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9485830Z 6 LL |     ) -> &dyn Foo
2019-10-02T17:39:57.9486384Z -    |          ^^^^^^^^
2019-10-02T17:39:57.9487191Z -    |          |
2019-10-02T17:39:57.9487191Z -    |          |
2019-10-02T17:39:57.9487465Z -    |          ...but data from `foo` is returned here
2019-10-02T17:39:57.9487891Z + LL |     {
2019-10-02T17:39:57.9487937Z + LL |         foo
2019-10-02T17:39:57.9487937Z + LL |         foo
2019-10-02T17:39:57.9487987Z +    |         ^^^ ...but data from `foo` is returned here
2019-10-02T17:39:57.9488104Z 11 error: aborting due to previous error
2019-10-02T17:39:57.9488147Z 12 
2019-10-02T17:39:57.9488196Z 
2019-10-02T17:39:57.9488223Z 
2019-10-02T17:39:57.9488223Z 
2019-10-02T17:39:57.9488271Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9488631Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1/issue-63388-1.stderr
2019-10-02T17:39:57.9488923Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9489204Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`
2019-10-02T17:39:57.9489308Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9489356Z status: exit code: 1
2019-10-02T17:39:57.9489356Z status: exit code: 1
2019-10-02T17:39:57.9490149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9490623Z ------------------------------------------
2019-10-02T17:39:57.9490668Z 
2019-10-02T17:39:57.9490843Z ------------------------------------------
2019-10-02T17:39:57.9490882Z stderr:
2019-10-02T17:39:57.9490882Z stderr:
2019-10-02T17:39:57.9491068Z ------------------------------------------
2019-10-02T17:39:57.9491108Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9491489Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:9
2019-10-02T17:39:57.9491532Z    |
2019-10-02T17:39:57.9491720Z LL |         &'a self, foo: &dyn Foo
2019-10-02T17:39:57.9491930Z    |         -------- this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9492131Z LL |     ) -> &dyn Foo //~ ERROR lifetime mismatch
2019-10-02T17:39:57.9492328Z LL |     {
2019-10-02T17:39:57.9492362Z LL |         foo
2019-10-02T17:39:57.9492362Z LL |         foo
2019-10-02T17:39:57.9492418Z    |         ^^^ ...but data from `foo` is returned here
2019-10-02T17:39:57.9492535Z error: aborting due to previous error
2019-10-02T17:39:57.9492559Z 
2019-10-02T17:39:57.9492598Z 
2019-10-02T17:39:57.9492774Z ------------------------------------------
2019-10-02T17:39:57.9492774Z ------------------------------------------
2019-10-02T17:39:57.9492800Z 
2019-10-02T17:39:57.9492820Z 
2019-10-02T17:39:57.9493020Z ---- [ui] ui/async-await/issues/issue-63388-2.rs stdout ----
2019-10-02T17:39:57.9493066Z diff of stderr:
2019-10-02T17:39:57.9493089Z 
2019-10-02T17:39:57.9493120Z 11    |
2019-10-02T17:39:57.9493316Z 12 LL |         foo: &dyn Foo, bar: &'a dyn Foo
2019-10-02T17:39:57.9493357Z 13    |         ^^^ ...but this borrow...
2019-10-02T17:39:57.9493511Z - LL |     ) -> &dyn Foo
2019-10-02T17:39:57.9493724Z -    |          -------- this return type evaluates to the `'static` lifetime...
2019-10-02T17:39:57.9493764Z + ...
2019-10-02T17:39:57.9493798Z + LL |         foo
2019-10-02T17:39:57.9494007Z +    |         --- this return type evaluates to the `'static` lifetime...
2019-10-02T17:39:57.9494047Z 16    |
2019-10-02T17:39:57.9494243Z 17 note: ...can't outlive the lifetime '_ as defined on the method body at 11:14
2019-10-02T17:39:57.9494537Z 18   --> $DIR/issue-63388-2.rs:11:14
2019-10-02T17:39:57.9494616Z 21    |              ^
2019-10-02T17:39:57.9494616Z 21    |              ^
2019-10-02T17:39:57.9494943Z 22 help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 11:14
2019-10-02T17:39:57.9495017Z 23    |
2019-10-02T17:39:57.9495206Z - LL |     ) -> &dyn Foo + '_
2019-10-02T17:39:57.9495366Z -    |          ^^^^^^^^^^^^^
2019-10-02T17:39:57.9495539Z + LL |         foo + '_
2019-10-02T17:39:57.9495608Z 26 
2019-10-02T17:39:57.9495644Z 27 error: aborting due to 2 previous errors
2019-10-02T17:39:57.9495695Z 28 
2019-10-02T17:39:57.9495716Z 
2019-10-02T17:39:57.9495716Z 
2019-10-02T17:39:57.9495737Z 
2019-10-02T17:39:57.9495773Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9496219Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2/issue-63388-2.stderr
2019-10-02T17:39:57.9497231Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9497521Z To only update this specific test, also pass `--test-args async-await/issues/issue-63388-2.rs`
2019-10-02T17:39:57.9497636Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9497683Z status: exit code: 1
2019-10-02T17:39:57.9497683Z status: exit code: 1
2019-10-02T17:39:57.9498489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-2/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9498835Z ------------------------------------------
2019-10-02T17:39:57.9498870Z 
2019-10-02T17:39:57.9499089Z ------------------------------------------
2019-10-02T17:39:57.9499153Z stderr:
2019-10-02T17:39:57.9499153Z stderr:
2019-10-02T17:39:57.9499406Z ------------------------------------------
2019-10-02T17:39:57.9499468Z error[E0106]: missing lifetime specifier
2019-10-02T17:39:57.9499730Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-2.rs:12:10
2019-10-02T17:39:57.9499812Z    |
2019-10-02T17:39:57.9500393Z LL |     ) -> &dyn Foo //~ ERROR missing lifetime specifier
2019-10-02T17:39:57.9500597Z    |          ^ help: consider using the named lifetime: `&'a`
2019-10-02T17:39:57.9500655Z    |
2019-10-02T17:39:57.9500917Z    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `foo` or `bar`
2019-10-02T17:39:57.9501014Z error: cannot infer an appropriate lifetime
2019-10-02T17:39:57.9501391Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-2.rs:11:9
2019-10-02T17:39:57.9501431Z    |
2019-10-02T17:39:57.9501431Z    |
2019-10-02T17:39:57.9501929Z LL |         foo: &dyn Foo, bar: &'a dyn Foo //~ ERROR cannot infer
2019-10-02T17:39:57.9501975Z    |         ^^^ ...but this borrow...
2019-10-02T17:39:57.9502072Z LL |         foo
2019-10-02T17:39:57.9502072Z LL |         foo
2019-10-02T17:39:57.9502285Z    |         --- this return type evaluates to the `'static` lifetime...
2019-10-02T17:39:57.9502327Z    |
2019-10-02T17:39:57.9502538Z note: ...can't outlive the lifetime '_ as defined on the method body at 11:14
2019-10-02T17:39:57.9502932Z   --> /checkout/src/test/ui/async-await/issues/issue-63388-2.rs:11:14
2019-10-02T17:39:57.9502973Z    |
2019-10-02T17:39:57.9503167Z LL |         foo: &dyn Foo, bar: &'a dyn Foo //~ ERROR cannot infer
2019-10-02T17:39:57.9503226Z    |              ^
2019-10-02T17:39:57.9503489Z help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 11:14
2019-10-02T17:39:57.9503859Z LL |         foo + '_
2019-10-02T17:39:57.9503897Z    |
2019-10-02T17:39:57.9503920Z 
2019-10-02T17:39:57.9503956Z error: aborting due to 2 previous errors
---
2019-10-02T17:39:57.9505082Z 
2019-10-02T17:39:57.9505269Z ---- [ui] ui/async-await/unresolved_type_param.rs stdout ----
2019-10-02T17:39:57.9505309Z diff of stderr:
2019-10-02T17:39:57.9505349Z 
2019-10-02T17:39:57.9505546Z - error[E0698]: type inside `async` object must be known in this context
2019-10-02T17:39:57.9505592Z + error[E0698]: type inside `async` fn body must be known in this context
2019-10-02T17:39:57.9505830Z 3    |
2019-10-02T17:39:57.9505830Z 3    |
2019-10-02T17:39:57.9505865Z 4 LL |     bar().await;
2019-10-02T17:39:57.9505941Z 5    |     ^^^ cannot infer type for `T`
2019-10-02T17:39:57.9506159Z 6    |
2019-10-02T17:39:57.9506159Z 6    |
2019-10-02T17:39:57.9506542Z - note: the type is part of the `async` object because of this `await`
2019-10-02T17:39:57.9506960Z + note: the type is part of the `async` fn body because of this `await`
2019-10-02T17:39:57.9507290Z 9    |
2019-10-02T17:39:57.9507290Z 9    |
2019-10-02T17:39:57.9507334Z 10 LL |     bar().await;
2019-10-02T17:39:57.9507489Z 
2019-10-02T17:39:57.9507535Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9507857Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/unresolved_type_param.stderr
2019-10-02T17:39:57.9507857Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/unresolved_type_param.stderr
2019-10-02T17:39:57.9508132Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9509136Z To only update this specific test, also pass `--test-args async-await/unresolved_type_param.rs`
2019-10-02T17:39:57.9509252Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9509299Z status: exit code: 1
2019-10-02T17:39:57.9509299Z status: exit code: 1
2019-10-02T17:39:57.9510095Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unresolved_type_param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9510561Z ------------------------------------------
2019-10-02T17:39:57.9510613Z 
2019-10-02T17:39:57.9510793Z ------------------------------------------
2019-10-02T17:39:57.9510831Z stderr:
2019-10-02T17:39:57.9510831Z stderr:
2019-10-02T17:39:57.9511015Z ------------------------------------------
2019-10-02T17:39:57.9511059Z error[E0698]: type inside `async` fn body must be known in this context
2019-10-02T17:39:57.9511321Z    |
2019-10-02T17:39:57.9511321Z    |
2019-10-02T17:39:57.9511356Z LL |     bar().await;
2019-10-02T17:39:57.9511428Z    |
2019-10-02T17:39:57.9511428Z    |
2019-10-02T17:39:57.9511485Z note: the type is part of the `async` fn body because of this `await`
2019-10-02T17:39:57.9511731Z    |
2019-10-02T17:39:57.9511731Z    |
2019-10-02T17:39:57.9511782Z LL |     bar().await;
2019-10-02T17:39:57.9511840Z 
2019-10-02T17:39:57.9511875Z error: aborting due to previous error
2019-10-02T17:39:57.9512045Z 
2019-10-02T17:39:57.9512276Z For more information about this error, try `rustc --explain E0698`.
---
2019-10-02T17:39:57.9512955Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9513182Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:45
2019-10-02T17:39:57.9513567Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:52
2019-10-02T17:39:57.9513605Z 3    |
2019-10-02T17:39:57.9513787Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-10-02T17:39:57.9514148Z -    |                          ----               ^^^^
2019-10-02T17:39:57.9514352Z -    |                          |                  |
2019-10-02T17:39:57.9514573Z -    |                          |                  ...but data from `f` is returned here
2019-10-02T17:39:57.9514791Z +    |                          ----               ----   ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9514910Z 8    |                          this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9514967Z 9 
2019-10-02T17:39:57.9515004Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9515029Z 
2019-10-02T17:39:57.9515029Z 
2019-10-02T17:39:57.9515230Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:55
2019-10-02T17:39:57.9515619Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:82
2019-10-02T17:39:57.9515660Z 12    |
2019-10-02T17:39:57.9515877Z 13 LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-10-02T17:39:57.9516103Z -    |                          -----                        ^^^^^^^^^^^^^^^^^
2019-10-02T17:39:57.9516660Z -    |                          |                            |
2019-10-02T17:39:57.9516977Z -    |                          |                            ...but data from `f` is returned here
2019-10-02T17:39:57.9517314Z +    |                          -----                        -----------------          ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9517436Z 17    |                          this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9517505Z 18 
2019-10-02T17:39:57.9517551Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9517582Z 
2019-10-02T17:39:57.9517582Z 
2019-10-02T17:39:57.9517837Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:58
2019-10-02T17:39:57.9518104Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
2019-10-02T17:39:57.9518154Z 21    |
2019-10-02T17:39:57.9518403Z 22 LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg }
2019-10-02T17:39:57.9518681Z -    |                                  -----                   ^^^
2019-10-02T17:39:57.9519206Z -    |                                  |                       ...but data from `arg` is returned here
2019-10-02T17:39:57.9519206Z -    |                                  |                       ...but data from `arg` is returned here
2019-10-02T17:39:57.9519529Z +    |                                  -----                   ---   ^^^ ...but data from `arg` is returned here
2019-10-02T17:39:57.9519646Z 26    |                                  this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9519715Z 27 
2019-10-02T17:39:57.9519762Z 28 error: aborting due to 3 previous errors
2019-10-02T17:39:57.9519792Z 
2019-10-02T17:39:57.9519792Z 
2019-10-02T17:39:57.9519819Z 
2019-10-02T17:39:57.9520040Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9520505Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async/arbitrary_self_types_pin_lifetime_mismatch-async.stderr
2019-10-02T17:39:57.9520871Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9521180Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-10-02T17:39:57.9521278Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9521315Z status: exit code: 1
2019-10-02T17:39:57.9521315Z status: exit code: 1
2019-10-02T17:39:57.9522164Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9522445Z ------------------------------------------
2019-10-02T17:39:57.9522489Z 
2019-10-02T17:39:57.9522665Z ------------------------------------------
2019-10-02T17:39:57.9522702Z stderr:
2019-10-02T17:39:57.9522702Z stderr:
2019-10-02T17:39:57.9522868Z ------------------------------------------
2019-10-02T17:39:57.9522924Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9523129Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:8:52
2019-10-02T17:39:57.9523170Z    |
2019-10-02T17:39:57.9523370Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-10-02T17:39:57.9523580Z    |                          ----               ----   ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9523868Z    |                          this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9523896Z 
2019-10-02T17:39:57.9523930Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9524165Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:82
2019-10-02T17:39:57.9524165Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:11:82
2019-10-02T17:39:57.9524213Z    |
2019-10-02T17:39:57.9524426Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-10-02T17:39:57.9524688Z    |                          -----                        -----------------          ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9524783Z    |                          this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9524828Z 
2019-10-02T17:39:57.9524863Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9525081Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
2019-10-02T17:39:57.9525081Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:17:64
2019-10-02T17:39:57.9525145Z    |
2019-10-02T17:39:57.9525355Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-10-02T17:39:57.9525584Z    |                                  -----                   ---   ^^^ ...but data from `arg` is returned here
2019-10-02T17:39:57.9525698Z    |                                  this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9525727Z 
2019-10-02T17:39:57.9525763Z error: aborting due to 3 previous errors
2019-10-02T17:39:57.9525806Z 
---
2019-10-02T17:39:57.9527527Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9527826Z -   --> $DIR/lt-ref-self-async.rs:13:42
2019-10-02T17:39:57.9528042Z +   --> $DIR/lt-ref-self-async.rs:14:9
2019-10-02T17:39:57.9528089Z 3    |
2019-10-02T17:39:57.9528341Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9528652Z -    |                       -----              ^^^^
2019-10-02T17:39:57.9528914Z -    |                       |                  |
2019-10-02T17:39:57.9529195Z -    |                       |                  ...but data from `f` is returned here
2019-10-02T17:39:57.9529428Z +    |                       -----              ----
2019-10-02T17:39:57.9529555Z 8    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9529608Z + LL |         f
2019-10-02T17:39:57.9529608Z + LL |         f
2019-10-02T17:39:57.9529656Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9529773Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9529995Z -   --> $DIR/lt-ref-self-async.rs:19:48
2019-10-02T17:39:57.9530210Z +   --> $DIR/lt-ref-self-async.rs:20:9
2019-10-02T17:39:57.9530499Z 12    |
2019-10-02T17:39:57.9530499Z 12    |
2019-10-02T17:39:57.9530864Z 13 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9531058Z -    |                             -----              ^^^^
2019-10-02T17:39:57.9531262Z -    |                             |                  |
2019-10-02T17:39:57.9531467Z -    |                             |                  ...but data from `f` is returned here
2019-10-02T17:39:57.9531649Z +    |                             -----              ----
2019-10-02T17:39:57.9531751Z 17    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9531790Z + LL |         f
2019-10-02T17:39:57.9531790Z + LL |         f
2019-10-02T17:39:57.9532012Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9532091Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9532460Z -   --> $DIR/lt-ref-self-async.rs:23:57
2019-10-02T17:39:57.9532629Z +   --> $DIR/lt-ref-self-async.rs:24:9
2019-10-02T17:39:57.9532666Z 21    |
2019-10-02T17:39:57.9532666Z 21    |
2019-10-02T17:39:57.9532860Z 22 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9533231Z -    |                                     -----               ^^^^
2019-10-02T17:39:57.9533412Z -    |                                     |                   |
2019-10-02T17:39:57.9533616Z -    |                                     |                   ...but data from `f` is returned here
2019-10-02T17:39:57.9533818Z +    |                                     -----               ----
2019-10-02T17:39:57.9533903Z 26    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9533966Z + LL |         f
2019-10-02T17:39:57.9533966Z + LL |         f
2019-10-02T17:39:57.9534001Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9534084Z 28 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9534257Z -   --> $DIR/lt-ref-self-async.rs:27:57
2019-10-02T17:39:57.9534425Z +   --> $DIR/lt-ref-self-async.rs:28:9
2019-10-02T17:39:57.9534477Z 30    |
2019-10-02T17:39:57.9534477Z 30    |
2019-10-02T17:39:57.9534668Z 31 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9534849Z -    |                                     -----               ^^^^
2019-10-02T17:39:57.9535045Z -    |                                     |                   |
2019-10-02T17:39:57.9535255Z -    |                                     |                   ...but data from `f` is returned here
2019-10-02T17:39:57.9535439Z +    |                                     -----               ----
2019-10-02T17:39:57.9535623Z 35    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9535662Z + LL |         f
2019-10-02T17:39:57.9535662Z + LL |         f
2019-10-02T17:39:57.9535714Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9535780Z 37 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9536227Z -   --> $DIR/lt-ref-self-async.rs:31:66
2019-10-02T17:39:57.9536852Z +   --> $DIR/lt-ref-self-async.rs:32:9
2019-10-02T17:39:57.9536907Z 39    |
2019-10-02T17:39:57.9536907Z 39    |
2019-10-02T17:39:57.9537167Z 40 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9537443Z -    |                                             -----                ^^^^
2019-10-02T17:39:57.9537693Z -    |                                             |                    |
2019-10-02T17:39:57.9537975Z -    |                                             |                    ...but data from `f` is returned here
2019-10-02T17:39:57.9538251Z +    |                                             -----                ----
2019-10-02T17:39:57.9538379Z 44    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9538451Z + LL |         f
2019-10-02T17:39:57.9538451Z + LL |         f
2019-10-02T17:39:57.9538506Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9538614Z 46 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9538843Z -   --> $DIR/lt-ref-self-async.rs:35:62
2019-10-02T17:39:57.9539060Z +   --> $DIR/lt-ref-self-async.rs:36:9
2019-10-02T17:39:57.9539125Z 48    |
2019-10-02T17:39:57.9539125Z 48    |
2019-10-02T17:39:57.9539374Z 49 LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9539622Z -    |                                         -----                ^^^^
2019-10-02T17:39:57.9540037Z -    |                                         |                    |
2019-10-02T17:39:57.9540275Z -    |                                         |                    ...but data from `f` is returned here
2019-10-02T17:39:57.9540464Z +    |                                         -----                ----
2019-10-02T17:39:57.9540573Z 53    |                                         this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9540613Z + LL |         f
2019-10-02T17:39:57.9540613Z + LL |         f
2019-10-02T17:39:57.9540649Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9540737Z 55 error: aborting due to 6 previous errors
2019-10-02T17:39:57.9540770Z 56 
2019-10-02T17:39:57.9540810Z 
2019-10-02T17:39:57.9540831Z 
2019-10-02T17:39:57.9540831Z 
2019-10-02T17:39:57.9540866Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9541110Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async/lt-ref-self-async.stderr
2019-10-02T17:39:57.9541326Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9541526Z To only update this specific test, also pass `--test-args self/elision/lt-ref-self-async.rs`
2019-10-02T17:39:57.9541605Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9541646Z status: exit code: 1
2019-10-02T17:39:57.9541646Z status: exit code: 1
2019-10-02T17:39:57.9542225Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-ref-self-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9542613Z ------------------------------------------
2019-10-02T17:39:57.9542657Z 
2019-10-02T17:39:57.9542827Z ------------------------------------------
2019-10-02T17:39:57.9542863Z stderr:
2019-10-02T17:39:57.9542863Z stderr:
2019-10-02T17:39:57.9543024Z ------------------------------------------
2019-10-02T17:39:57.9543080Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9543329Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:14:9
2019-10-02T17:39:57.9543377Z    |
2019-10-02T17:39:57.9543599Z LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9543947Z    |                       -----              ----
2019-10-02T17:39:57.9544048Z    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9544048Z    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9544249Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9544289Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9544416Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9544607Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:20:9
2019-10-02T17:39:57.9544662Z    |
2019-10-02T17:39:57.9544662Z    |
2019-10-02T17:39:57.9544837Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9545015Z    |                             -----              ----
2019-10-02T17:39:57.9545115Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9545115Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9545156Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9545195Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9545266Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9545456Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:24:9
2019-10-02T17:39:57.9545514Z    |
2019-10-02T17:39:57.9545514Z    |
2019-10-02T17:39:57.9545695Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9545886Z    |                                     -----               ----
2019-10-02T17:39:57.9546163Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9546163Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9546213Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9546251Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9546481Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9546952Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:28:9
2019-10-02T17:39:57.9547023Z    |
2019-10-02T17:39:57.9547023Z    |
2019-10-02T17:39:57.9547268Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9547508Z    |                                     -----               ----
2019-10-02T17:39:57.9547637Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9547637Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9547703Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9547772Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9547848Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9548102Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:32:9
2019-10-02T17:39:57.9548170Z    |
2019-10-02T17:39:57.9548170Z    |
2019-10-02T17:39:57.9553240Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9553496Z    |                                             -----                ----
2019-10-02T17:39:57.9553618Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9553618Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9553665Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9553724Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9553978Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9554227Z   --> /checkout/src/test/ui/self/elision/lt-ref-self-async.rs:36:9
2019-10-02T17:39:57.9554288Z    |
2019-10-02T17:39:57.9554288Z    |
2019-10-02T17:39:57.9554580Z LL |     async fn box_pin_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9554819Z    |                                         -----                ----
2019-10-02T17:39:57.9554932Z    |                                         this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9554932Z    |                                         this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9554978Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9555036Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9555100Z error: aborting due to 6 previous errors
2019-10-02T17:39:57.9555125Z 
2019-10-02T17:39:57.9555165Z 
2019-10-02T17:39:57.9557296Z ------------------------------------------
---
2019-10-02T17:39:57.9557871Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9558135Z -   --> $DIR/ref-mut-self-async.rs:13:46
2019-10-02T17:39:57.9558364Z +   --> $DIR/ref-mut-self-async.rs:14:9
2019-10-02T17:39:57.9558412Z 3    |
2019-10-02T17:39:57.9558650Z 4 LL |     async fn ref_self(&mut self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9558909Z -    |                       ---------              ^^^^
2019-10-02T17:39:57.9559144Z -    |                       |                      |
2019-10-02T17:39:57.9559409Z -    |                       |                      ...but data from `f` is returned here
2019-10-02T17:39:57.9559665Z +    |                       ---------              ----
2019-10-02T17:39:57.9560246Z 8    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9560333Z + LL |         f
2019-10-02T17:39:57.9560333Z + LL |         f
2019-10-02T17:39:57.9560373Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9560474Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9560719Z -   --> $DIR/ref-mut-self-async.rs:19:52
2019-10-02T17:39:57.9560903Z +   --> $DIR/ref-mut-self-async.rs:20:9
2019-10-02T17:39:57.9560960Z 12    |
2019-10-02T17:39:57.9560960Z 12    |
2019-10-02T17:39:57.9561162Z 13 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9561360Z -    |                             ---------              ^^^^
2019-10-02T17:39:57.9561556Z -    |                             |                      |
2019-10-02T17:39:57.9561797Z -    |                             |                      ...but data from `f` is returned here
2019-10-02T17:39:57.9562148Z +    |                             ---------              ----
2019-10-02T17:39:57.9562260Z 17    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9562301Z + LL |         f
2019-10-02T17:39:57.9562301Z + LL |         f
2019-10-02T17:39:57.9562512Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9562606Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9562787Z -   --> $DIR/ref-mut-self-async.rs:23:61
2019-10-02T17:39:57.9562978Z +   --> $DIR/ref-mut-self-async.rs:24:9
2019-10-02T17:39:57.9563015Z 21    |
2019-10-02T17:39:57.9563015Z 21    |
2019-10-02T17:39:57.9563207Z 22 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9563415Z -    |                                     ---------               ^^^^
2019-10-02T17:39:57.9563610Z -    |                                     |                       |
2019-10-02T17:39:57.9563826Z -    |                                     |                       ...but data from `f` is returned here
2019-10-02T17:39:57.9564683Z +    |                                     ---------               ----
2019-10-02T17:39:57.9564861Z 26    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9564928Z + LL |         f
2019-10-02T17:39:57.9564928Z + LL |         f
2019-10-02T17:39:57.9564964Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9565052Z 28 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9565253Z -   --> $DIR/ref-mut-self-async.rs:27:61
2019-10-02T17:39:57.9565419Z +   --> $DIR/ref-mut-self-async.rs:28:9
2019-10-02T17:39:57.9565454Z 30    |
2019-10-02T17:39:57.9565454Z 30    |
2019-10-02T17:39:57.9565659Z 31 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9565846Z -    |                                     ---------               ^^^^
2019-10-02T17:39:57.9566204Z -    |                                     |                       |
2019-10-02T17:39:57.9566881Z -    |                                     |                       ...but data from `f` is returned here
2019-10-02T17:39:57.9567145Z +    |                                     ---------               ----
2019-10-02T17:39:57.9567288Z 35    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9567341Z + LL |         f
2019-10-02T17:39:57.9567341Z + LL |         f
2019-10-02T17:39:57.9567389Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9567499Z 37 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9567731Z -   --> $DIR/ref-mut-self-async.rs:31:70
2019-10-02T17:39:57.9567969Z +   --> $DIR/ref-mut-self-async.rs:32:9
2019-10-02T17:39:57.9568016Z 39    |
2019-10-02T17:39:57.9568016Z 39    |
2019-10-02T17:39:57.9568273Z 40 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9568561Z -    |                                             ---------                ^^^^
2019-10-02T17:39:57.9568817Z -    |                                             |                        |
2019-10-02T17:39:57.9569106Z -    |                                             |                        ...but data from `f` is returned here
2019-10-02T17:39:57.9569390Z +    |                                             ---------                ----
2019-10-02T17:39:57.9569508Z 44    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9569579Z + LL |         f
2019-10-02T17:39:57.9569579Z + LL |         f
2019-10-02T17:39:57.9569628Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9572945Z 46 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9573468Z -   --> $DIR/ref-mut-self-async.rs:35:70
2019-10-02T17:39:57.9573667Z +   --> $DIR/ref-mut-self-async.rs:36:9
2019-10-02T17:39:57.9573705Z 48    |
2019-10-02T17:39:57.9573705Z 48    |
2019-10-02T17:39:57.9573927Z 49 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9574128Z -    |                                             ---------                ^^^^
2019-10-02T17:39:57.9574331Z -    |                                             |                        |
2019-10-02T17:39:57.9574581Z -    |                                             |                        ...but data from `f` is returned here
2019-10-02T17:39:57.9574784Z +    |                                             ---------                ----
2019-10-02T17:39:57.9574896Z 53    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9574938Z + LL |         f
2019-10-02T17:39:57.9574938Z + LL |         f
2019-10-02T17:39:57.9574975Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9575230Z 55 error: aborting due to 6 previous errors
2019-10-02T17:39:57.9575264Z 56 
2019-10-02T17:39:57.9575306Z 
2019-10-02T17:39:57.9575328Z 
2019-10-02T17:39:57.9575328Z 
2019-10-02T17:39:57.9575366Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9575904Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async/ref-mut-self-async.stderr
2019-10-02T17:39:57.9576922Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9577298Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-10-02T17:39:57.9577405Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9577453Z status: exit code: 1
2019-10-02T17:39:57.9577453Z status: exit code: 1
2019-10-02T17:39:57.9578252Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9578605Z ------------------------------------------
2019-10-02T17:39:57.9578658Z 
2019-10-02T17:39:57.9578884Z ------------------------------------------
2019-10-02T17:39:57.9578933Z stderr:
2019-10-02T17:39:57.9578933Z stderr:
2019-10-02T17:39:57.9579165Z ------------------------------------------
2019-10-02T17:39:57.9579217Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9579465Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:14:9
2019-10-02T17:39:57.9579518Z    |
2019-10-02T17:39:57.9580167Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-10-02T17:39:57.9580392Z    |                       ---------              ----
2019-10-02T17:39:57.9580506Z    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9580553Z LL |         f
2019-10-02T17:39:57.9580553Z LL |         f
2019-10-02T17:39:57.9580604Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9580691Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9580915Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:20:9
2019-10-02T17:39:57.9580977Z    |
2019-10-02T17:39:57.9580977Z    |
2019-10-02T17:39:57.9581187Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9581393Z    |                             ---------              ----
2019-10-02T17:39:57.9581561Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9581561Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9581618Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9581680Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9581746Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9582128Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:24:9
2019-10-02T17:39:57.9582188Z    |
2019-10-02T17:39:57.9582188Z    |
2019-10-02T17:39:57.9582751Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9582944Z    |                                     ---------               ----
2019-10-02T17:39:57.9583049Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9583049Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9583093Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9583150Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9583332Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9583562Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:28:9
2019-10-02T17:39:57.9583619Z    |
2019-10-02T17:39:57.9583619Z    |
2019-10-02T17:39:57.9583817Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9584403Z    |                                     ---------               ----
2019-10-02T17:39:57.9584510Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9584510Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9584550Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9584605Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9584661Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9584867Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:32:9
2019-10-02T17:39:57.9584921Z    |
2019-10-02T17:39:57.9584921Z    |
2019-10-02T17:39:57.9585106Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9585485Z    |                                             ---------                ----
2019-10-02T17:39:57.9585597Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9585597Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9585640Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9585927Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9585987Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9586844Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:36:9
2019-10-02T17:39:57.9586904Z    |
2019-10-02T17:39:57.9586904Z    |
2019-10-02T17:39:57.9587208Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9587469Z    |                                             ---------                ----
2019-10-02T17:39:57.9587613Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9587613Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9587685Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9587737Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9587823Z error: aborting due to 6 previous errors
2019-10-02T17:39:57.9587875Z 
2019-10-02T17:39:57.9587902Z 
2019-10-02T17:39:57.9588132Z ------------------------------------------
---
2019-10-02T17:39:57.9597247Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9597534Z -   --> $DIR/ref-mut-struct-async.rs:13:56
2019-10-02T17:39:57.9597758Z +   --> $DIR/ref-mut-struct-async.rs:14:9
2019-10-02T17:39:57.9597826Z 3    |
2019-10-02T17:39:57.9598095Z 4 LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-10-02T17:39:57.9598343Z -    |                               -----------              ^^^^
2019-10-02T17:39:57.9598584Z -    |                               |                        |
2019-10-02T17:39:57.9598886Z -    |                               |                        ...but data from `f` is returned here
2019-10-02T17:39:57.9599141Z +    |                               -----------              ----
2019-10-02T17:39:57.9599525Z 8    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9599592Z + LL |         f
2019-10-02T17:39:57.9599592Z + LL |         f
2019-10-02T17:39:57.9599643Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9599757Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9600192Z -   --> $DIR/ref-mut-struct-async.rs:17:65
2019-10-02T17:39:57.9600591Z +   --> $DIR/ref-mut-struct-async.rs:18:9
2019-10-02T17:39:57.9600649Z 12    |
2019-10-02T17:39:57.9600649Z 12    |
2019-10-02T17:39:57.9600849Z 13 LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9601045Z -    |                                       -----------               ^^^^
2019-10-02T17:39:57.9601343Z -    |                                       |                         |
2019-10-02T17:39:57.9601603Z -    |                                       |                         ...but data from `f` is returned here
2019-10-02T17:39:57.9601954Z +    |                                       -----------               ----
2019-10-02T17:39:57.9602057Z 17    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9602096Z + LL |         f
2019-10-02T17:39:57.9602096Z + LL |         f
2019-10-02T17:39:57.9602149Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9602224Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9602410Z -   --> $DIR/ref-mut-struct-async.rs:21:65
2019-10-02T17:39:57.9602575Z +   --> $DIR/ref-mut-struct-async.rs:22:9
2019-10-02T17:39:57.9602610Z 21    |
2019-10-02T17:39:57.9602610Z 21    |
2019-10-02T17:39:57.9602821Z 22 LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9603010Z -    |                                       -----------               ^^^^
2019-10-02T17:39:57.9603371Z -    |                                       |                         |
2019-10-02T17:39:57.9603607Z -    |                                       |                         ...but data from `f` is returned here
2019-10-02T17:39:57.9604009Z +    |                                       -----------               ----
2019-10-02T17:39:57.9604122Z 26    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9604174Z + LL |         f
2019-10-02T17:39:57.9604174Z + LL |         f
2019-10-02T17:39:57.9604211Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9604300Z 28 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9604495Z -   --> $DIR/ref-mut-struct-async.rs:25:74
2019-10-02T17:39:57.9604669Z +   --> $DIR/ref-mut-struct-async.rs:26:9
2019-10-02T17:39:57.9604722Z 30    |
2019-10-02T17:39:57.9604722Z 30    |
2019-10-02T17:39:57.9604926Z 31 LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9605128Z -    |                                               -----------                ^^^^
2019-10-02T17:39:57.9605347Z -    |                                               |                          |
2019-10-02T17:39:57.9605575Z -    |                                               |                          ...but data from `f` is returned here
2019-10-02T17:39:57.9605777Z +    |                                               -----------                ----
2019-10-02T17:39:57.9605891Z 35    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9605936Z + LL |         f
2019-10-02T17:39:57.9605936Z + LL |         f
2019-10-02T17:39:57.9606182Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9606254Z 37 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9608103Z -   --> $DIR/ref-mut-struct-async.rs:29:74
2019-10-02T17:39:57.9608356Z +   --> $DIR/ref-mut-struct-async.rs:30:9
2019-10-02T17:39:57.9608406Z 39    |
2019-10-02T17:39:57.9608406Z 39    |
2019-10-02T17:39:57.9608700Z 40 LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9608963Z -    |                                               -----------                ^^^^
2019-10-02T17:39:57.9609222Z -    |                                               |                          |
2019-10-02T17:39:57.9609771Z -    |                                               |                          ...but data from `f` is returned here
2019-10-02T17:39:57.9610040Z +    |                                               -----------                ----
2019-10-02T17:39:57.9610423Z 44    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9610461Z + LL |         f
2019-10-02T17:39:57.9610461Z + LL |         f
2019-10-02T17:39:57.9610497Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9610582Z 46 error: aborting due to 5 previous errors
2019-10-02T17:39:57.9610615Z 47 
2019-10-02T17:39:57.9610636Z 
2019-10-02T17:39:57.9610673Z 
2019-10-02T17:39:57.9610673Z 
2019-10-02T17:39:57.9610707Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9610972Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async/ref-mut-struct-async.stderr
2019-10-02T17:39:57.9611189Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9611395Z To only update this specific test, also pass `--test-args self/elision/ref-mut-struct-async.rs`
2019-10-02T17:39:57.9611463Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9611517Z status: exit code: 1
2019-10-02T17:39:57.9611517Z status: exit code: 1
2019-10-02T17:39:57.9612089Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9612353Z ------------------------------------------
2019-10-02T17:39:57.9612379Z 
2019-10-02T17:39:57.9612562Z ------------------------------------------
2019-10-02T17:39:57.9612597Z stderr:
2019-10-02T17:39:57.9612597Z stderr:
2019-10-02T17:39:57.9612760Z ------------------------------------------
2019-10-02T17:39:57.9612815Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9613002Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:14:9
2019-10-02T17:39:57.9613043Z    |
2019-10-02T17:39:57.9613244Z LL |     async fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-10-02T17:39:57.9613425Z    |                               -----------              ----
2019-10-02T17:39:57.9613524Z    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9613524Z    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9613567Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9613612Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9613685Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9613872Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:18:9
2019-10-02T17:39:57.9613909Z    |
2019-10-02T17:39:57.9613909Z    |
2019-10-02T17:39:57.9614116Z LL |     async fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9614305Z    |                                       -----------               ----
2019-10-02T17:39:57.9614404Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9614404Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9614445Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9614481Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9614553Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9614852Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:22:9
2019-10-02T17:39:57.9614907Z    |
2019-10-02T17:39:57.9614907Z    |
2019-10-02T17:39:57.9615091Z LL |     async fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9615275Z    |                                       -----------               ----
2019-10-02T17:39:57.9615448Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9615448Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9615488Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9615524Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9615603Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9615815Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:26:9
2019-10-02T17:39:57.9615868Z    |
2019-10-02T17:39:57.9615868Z    |
2019-10-02T17:39:57.9616241Z LL |     async fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9617054Z    |                                               -----------                ----
2019-10-02T17:39:57.9617216Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9617216Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9617272Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9617341Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9617417Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9617684Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct-async.rs:30:9
2019-10-02T17:39:57.9617752Z    |
2019-10-02T17:39:57.9617752Z    |
2019-10-02T17:39:57.9618011Z LL |     async fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9618268Z    |                                               -----------                ----
2019-10-02T17:39:57.9618409Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9618409Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9618465Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9618534Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9618618Z error: aborting due to 5 previous errors
2019-10-02T17:39:57.9618649Z 
2019-10-02T17:39:57.9618676Z 
2019-10-02T17:39:57.9618923Z ------------------------------------------
---
2019-10-02T17:39:57.9619354Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9619591Z -   --> $DIR/ref-self-async.rs:22:42
2019-10-02T17:39:57.9619964Z +   --> $DIR/ref-self-async.rs:23:9
2019-10-02T17:39:57.9620195Z 3    |
2019-10-02T17:39:57.9620378Z 4 LL |     async fn ref_self(&self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9620576Z -    |                       -----              ^^^^
2019-10-02T17:39:57.9620751Z -    |                       |                  |
2019-10-02T17:39:57.9620954Z -    |                       |                  ...but data from `f` is returned here
2019-10-02T17:39:57.9621160Z +    |                       -----              ----
2019-10-02T17:39:57.9621242Z 8    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9621299Z + LL |         f
2019-10-02T17:39:57.9621299Z + LL |         f
2019-10-02T17:39:57.9621336Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9621424Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9621597Z -   --> $DIR/ref-self-async.rs:28:48
2019-10-02T17:39:57.9621763Z +   --> $DIR/ref-self-async.rs:29:9
2019-10-02T17:39:57.9621914Z 12    |
2019-10-02T17:39:57.9621914Z 12    |
2019-10-02T17:39:57.9622152Z 13 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9622341Z -    |                             -----              ^^^^
2019-10-02T17:39:57.9622521Z -    |                             |                  |
2019-10-02T17:39:57.9622829Z -    |                             |                  ...but data from `f` is returned here
2019-10-02T17:39:57.9623051Z +    |                             -----              ----
2019-10-02T17:39:57.9623313Z 17    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9623352Z + LL |         f
2019-10-02T17:39:57.9623352Z + LL |         f
2019-10-02T17:39:57.9623386Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9623469Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9623633Z -   --> $DIR/ref-self-async.rs:32:57
2019-10-02T17:39:57.9623807Z +   --> $DIR/ref-self-async.rs:33:9
2019-10-02T17:39:57.9623849Z 21    |
2019-10-02T17:39:57.9623849Z 21    |
2019-10-02T17:39:57.9624035Z 22 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9624233Z -    |                                     -----               ^^^^
2019-10-02T17:39:57.9624414Z -    |                                     |                   |
2019-10-02T17:39:57.9624621Z -    |                                     |                   ...but data from `f` is returned here
2019-10-02T17:39:57.9624822Z +    |                                     -----               ----
2019-10-02T17:39:57.9624906Z 26    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9624961Z + LL |         f
2019-10-02T17:39:57.9624961Z + LL |         f
2019-10-02T17:39:57.9624997Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9625062Z 28 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9625251Z -   --> $DIR/ref-self-async.rs:36:57
2019-10-02T17:39:57.9625410Z +   --> $DIR/ref-self-async.rs:37:9
2019-10-02T17:39:57.9625445Z 30    |
2019-10-02T17:39:57.9625445Z 30    |
2019-10-02T17:39:57.9625641Z 31 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9625826Z -    |                                     -----               ^^^^
2019-10-02T17:39:57.9626184Z -    |                                     |                   |
2019-10-02T17:39:57.9626820Z -    |                                     |                   ...but data from `f` is returned here
2019-10-02T17:39:57.9627084Z +    |                                     -----               ----
2019-10-02T17:39:57.9627217Z 35    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9627270Z + LL |         f
2019-10-02T17:39:57.9627270Z + LL |         f
2019-10-02T17:39:57.9627318Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9627438Z 37 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9627660Z -   --> $DIR/ref-self-async.rs:40:66
2019-10-02T17:39:57.9627874Z +   --> $DIR/ref-self-async.rs:41:9
2019-10-02T17:39:57.9627940Z 39    |
2019-10-02T17:39:57.9627940Z 39    |
2019-10-02T17:39:57.9628201Z 40 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9628752Z -    |                                             -----                ^^^^
2019-10-02T17:39:57.9629028Z -    |                                             |                    |
2019-10-02T17:39:57.9629312Z -    |                                             |                    ...but data from `f` is returned here
2019-10-02T17:39:57.9629565Z +    |                                             -----                ----
2019-10-02T17:39:57.9629701Z 44    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9630052Z + LL |         f
2019-10-02T17:39:57.9630052Z + LL |         f
2019-10-02T17:39:57.9630103Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9630167Z 46 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9630384Z -   --> $DIR/ref-self-async.rs:44:66
2019-10-02T17:39:57.9630615Z +   --> $DIR/ref-self-async.rs:45:9
2019-10-02T17:39:57.9630658Z 48    |
2019-10-02T17:39:57.9630658Z 48    |
2019-10-02T17:39:57.9630889Z 49 LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9631080Z -    |                                             -----                ^^^^
2019-10-02T17:39:57.9631263Z -    |                                             |                    |
2019-10-02T17:39:57.9631484Z -    |                                             |                    ...but data from `f` is returned here
2019-10-02T17:39:57.9631675Z +    |                                             -----                ----
2019-10-02T17:39:57.9631782Z 53    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9631820Z + LL |         f
2019-10-02T17:39:57.9631820Z + LL |         f
2019-10-02T17:39:57.9631855Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9631943Z 55 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9632112Z -   --> $DIR/ref-self-async.rs:48:69
2019-10-02T17:39:57.9632269Z +   --> $DIR/ref-self-async.rs:49:9
2019-10-02T17:39:57.9632321Z 57    |
2019-10-02T17:39:57.9632321Z 57    |
2019-10-02T17:39:57.9632507Z 58 LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-10-02T17:39:57.9632694Z -    |                                            -----                    ^^^
2019-10-02T17:39:57.9632896Z -    |                                            |                        |
2019-10-02T17:39:57.9633107Z -    |                                            |                        ...but data from `f` is returned here
2019-10-02T17:39:57.9633306Z +    |                                            -----                    ---
2019-10-02T17:39:57.9633414Z 62    |                                            this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9633452Z + LL |         f
2019-10-02T17:39:57.9633452Z + LL |         f
2019-10-02T17:39:57.9633504Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9633570Z 64 error: aborting due to 7 previous errors
2019-10-02T17:39:57.9633622Z 65 
2019-10-02T17:39:57.9633644Z 
2019-10-02T17:39:57.9633664Z 
2019-10-02T17:39:57.9633664Z 
2019-10-02T17:39:57.9633698Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9633944Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async/ref-self-async.stderr
2019-10-02T17:39:57.9634129Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9634330Z To only update this specific test, also pass `--test-args self/elision/ref-self-async.rs`
2019-10-02T17:39:57.9634406Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9634441Z status: exit code: 1
2019-10-02T17:39:57.9634441Z status: exit code: 1
2019-10-02T17:39:57.9635211Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9635481Z ------------------------------------------
2019-10-02T17:39:57.9635584Z 
2019-10-02T17:39:57.9635784Z ------------------------------------------
2019-10-02T17:39:57.9635822Z stderr:
2019-10-02T17:39:57.9635822Z stderr:
2019-10-02T17:39:57.9636183Z ------------------------------------------
2019-10-02T17:39:57.9636224Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9636915Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:23:9
2019-10-02T17:39:57.9637007Z    |
2019-10-02T17:39:57.9637302Z LL |     async fn ref_self(&self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-10-02T17:39:57.9637535Z    |                       -----              ----
2019-10-02T17:39:57.9637662Z    |                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9637717Z LL |         f
2019-10-02T17:39:57.9637717Z LL |         f
2019-10-02T17:39:57.9637785Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9637861Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9638118Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:29:9
2019-10-02T17:39:57.9638187Z    |
2019-10-02T17:39:57.9638187Z    |
2019-10-02T17:39:57.9638419Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-10-02T17:39:57.9638650Z    |                             -----              ----
2019-10-02T17:39:57.9638785Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9638785Z    |                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9638841Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9638911Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9638987Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9639232Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:33:9
2019-10-02T17:39:57.9639301Z    |
2019-10-02T17:39:57.9639301Z    |
2019-10-02T17:39:57.9639547Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9639952Z    |                                     -----               ----
2019-10-02T17:39:57.9640242Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9640242Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9640285Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9640504Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9640559Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9640761Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:37:9
2019-10-02T17:39:57.9640798Z    |
2019-10-02T17:39:57.9640798Z    |
2019-10-02T17:39:57.9640974Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9641149Z    |                                     -----               ----
2019-10-02T17:39:57.9641246Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9641246Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9641292Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9641345Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9641398Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9641600Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:41:9
2019-10-02T17:39:57.9641637Z    |
2019-10-02T17:39:57.9641637Z    |
2019-10-02T17:39:57.9641824Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9642024Z    |                                             -----                ----
2019-10-02T17:39:57.9642108Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9642108Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9642166Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9642202Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9642355Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9642580Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:45:9
2019-10-02T17:39:57.9642617Z    |
2019-10-02T17:39:57.9642617Z    |
2019-10-02T17:39:57.9642802Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9643065Z    |                                             -----                ----
2019-10-02T17:39:57.9643156Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9643156Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9643213Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9643249Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9643303Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9643525Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:49:9
2019-10-02T17:39:57.9643562Z    |
2019-10-02T17:39:57.9643562Z    |
2019-10-02T17:39:57.9643756Z LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-10-02T17:39:57.9643959Z    |                                            -----                    ---
2019-10-02T17:39:57.9644049Z    |                                            this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9644049Z    |                                            this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9644106Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9644142Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9644198Z error: aborting due to 7 previous errors
2019-10-02T17:39:57.9644237Z 
2019-10-02T17:39:57.9644257Z 
2019-10-02T17:39:57.9644424Z ------------------------------------------
---
2019-10-02T17:39:57.9644772Z 1 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9644938Z -   --> $DIR/ref-struct-async.rs:13:52
2019-10-02T17:39:57.9645096Z +   --> $DIR/ref-struct-async.rs:14:9
2019-10-02T17:39:57.9645131Z 3    |
2019-10-02T17:39:57.9645324Z 4 LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-10-02T17:39:57.9645506Z -    |                               -------              ^^^^
2019-10-02T17:39:57.9645683Z -    |                               |                    |
2019-10-02T17:39:57.9645901Z -    |                               |                    ...but data from `f` is returned here
2019-10-02T17:39:57.9646258Z +    |                               -------              ----
2019-10-02T17:39:57.9646672Z 8    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9646727Z + LL |         f
2019-10-02T17:39:57.9646727Z + LL |         f
2019-10-02T17:39:57.9646786Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9646895Z 10 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9647166Z -   --> $DIR/ref-struct-async.rs:17:61
2019-10-02T17:39:57.9647399Z +   --> $DIR/ref-struct-async.rs:18:9
2019-10-02T17:39:57.9647447Z 12    |
2019-10-02T17:39:57.9647447Z 12    |
2019-10-02T17:39:57.9647704Z 13 LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9647973Z -    |                                       -------               ^^^^
2019-10-02T17:39:57.9648219Z -    |                                       |                     |
2019-10-02T17:39:57.9648503Z -    |                                       |                     ...but data from `f` is returned here
2019-10-02T17:39:57.9648768Z +    |                                       -------               ----
2019-10-02T17:39:57.9648882Z 17    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9649070Z + LL |         f
2019-10-02T17:39:57.9649070Z + LL |         f
2019-10-02T17:39:57.9649118Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9649210Z 19 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9649481Z -   --> $DIR/ref-struct-async.rs:21:61
2019-10-02T17:39:57.9649777Z +   --> $DIR/ref-struct-async.rs:22:9
2019-10-02T17:39:57.9649834Z 21    |
2019-10-02T17:39:57.9649834Z 21    |
2019-10-02T17:39:57.9650300Z 22 LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9650492Z -    |                                       -------               ^^^^
2019-10-02T17:39:57.9650671Z -    |                                       |                     |
2019-10-02T17:39:57.9650891Z -    |                                       |                     ...but data from `f` is returned here
2019-10-02T17:39:57.9651079Z +    |                                       -------               ----
2019-10-02T17:39:57.9651185Z 26    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9651223Z + LL |         f
2019-10-02T17:39:57.9651223Z + LL |         f
2019-10-02T17:39:57.9651258Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9651346Z 28 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9651515Z -   --> $DIR/ref-struct-async.rs:25:70
2019-10-02T17:39:57.9651691Z +   --> $DIR/ref-struct-async.rs:26:9
2019-10-02T17:39:57.9651726Z 30    |
2019-10-02T17:39:57.9651726Z 30    |
2019-10-02T17:39:57.9651914Z 31 LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9652103Z -    |                                               -------                ^^^^
2019-10-02T17:39:57.9652305Z -    |                                               |                      |
2019-10-02T17:39:57.9652517Z -    |                                               |                      ...but data from `f` is returned here
2019-10-02T17:39:57.9652714Z +    |                                               -------                ----
2019-10-02T17:39:57.9652821Z 35    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9652880Z + LL |         f
2019-10-02T17:39:57.9652880Z + LL |         f
2019-10-02T17:39:57.9652918Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9652984Z 37 error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9653170Z -   --> $DIR/ref-struct-async.rs:29:66
2019-10-02T17:39:57.9653329Z +   --> $DIR/ref-struct-async.rs:30:9
2019-10-02T17:39:57.9653364Z 39    |
2019-10-02T17:39:57.9653364Z 39    |
2019-10-02T17:39:57.9653566Z 40 LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9653749Z -    |                                           -------                ^^^^
2019-10-02T17:39:57.9653939Z -    |                                           |                      |
2019-10-02T17:39:57.9654165Z -    |                                           |                      ...but data from `f` is returned here
2019-10-02T17:39:57.9654350Z +    |                                           -------                ----
2019-10-02T17:39:57.9654456Z 44    |                                           this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9654494Z + LL |         f
2019-10-02T17:39:57.9654494Z + LL |         f
2019-10-02T17:39:57.9654528Z +    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9654613Z 46 error: aborting due to 5 previous errors
2019-10-02T17:39:57.9654645Z 47 
2019-10-02T17:39:57.9654665Z 
2019-10-02T17:39:57.9654702Z 
2019-10-02T17:39:57.9654702Z 
2019-10-02T17:39:57.9654736Z The actual stderr differed from the expected stderr.
2019-10-02T17:39:57.9655069Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async/ref-struct-async.stderr
2019-10-02T17:39:57.9655268Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T17:39:57.9655466Z To only update this specific test, also pass `--test-args self/elision/ref-struct-async.rs`
2019-10-02T17:39:57.9655603Z error: 1 errors occurred comparing output.
2019-10-02T17:39:57.9655654Z status: exit code: 1
2019-10-02T17:39:57.9655654Z status: exit code: 1
2019-10-02T17:39:57.9656381Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-struct-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-struct-async/auxiliary" "-A" "unused"
2019-10-02T17:39:57.9656947Z ------------------------------------------
2019-10-02T17:39:57.9656983Z 
2019-10-02T17:39:57.9657224Z ------------------------------------------
2019-10-02T17:39:57.9657273Z stderr:
2019-10-02T17:39:57.9657273Z stderr:
2019-10-02T17:39:57.9657496Z ------------------------------------------
2019-10-02T17:39:57.9657564Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9657814Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:14:9
2019-10-02T17:39:57.9657868Z    |
2019-10-02T17:39:57.9658127Z LL |     async fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-10-02T17:39:57.9658366Z    |                               -------              ----
2019-10-02T17:39:57.9658495Z    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9658495Z    |                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9658560Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9658612Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9658707Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9658955Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:18:9
2019-10-02T17:39:57.9659005Z    |
2019-10-02T17:39:57.9659005Z    |
2019-10-02T17:39:57.9659278Z LL |     async fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9659527Z    |                                       -------               ----
2019-10-02T17:39:57.9659658Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9659658Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9659713Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9659764Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9660025Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9660217Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:22:9
2019-10-02T17:39:57.9660254Z    |
2019-10-02T17:39:57.9660254Z    |
2019-10-02T17:39:57.9660451Z LL |     async fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9660630Z    |                                       -------               ----
2019-10-02T17:39:57.9660736Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9660736Z    |                                       this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9660776Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9660812Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9660884Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9661068Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:26:9
2019-10-02T17:39:57.9661121Z    |
2019-10-02T17:39:57.9661121Z    |
2019-10-02T17:39:57.9661308Z LL |     async fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9661619Z    |                                               -------                ----
2019-10-02T17:39:57.9661724Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9661724Z    |                                               this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9661833Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9661892Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9661947Z error[E0623]: lifetime mismatch
2019-10-02T17:39:57.9662151Z   --> /checkout/src/test/ui/self/elision/ref-struct-async.rs:30:9
2019-10-02T17:39:57.9662203Z    |
2019-10-02T17:39:57.9662203Z    |
2019-10-02T17:39:57.9662388Z LL |     async fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
2019-10-02T17:39:57.9662572Z    |                                           -------                ----
2019-10-02T17:39:57.9662680Z    |                                           this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9662680Z    |                                           this parameter and the return type are declared with different lifetimes...
2019-10-02T17:39:57.9662720Z LL |         f //~^ ERROR lifetime mismatch
2019-10-02T17:39:57.9662776Z    |         ^ ...but data from `f` is returned here
2019-10-02T17:39:57.9662837Z error: aborting due to 5 previous errors
2019-10-02T17:39:57.9662860Z 
2019-10-02T17:39:57.9662880Z 
2019-10-02T17:39:57.9663063Z ------------------------------------------
---
2019-10-02T17:39:57.9665533Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-02T17:39:57.9665579Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-02T17:39:57.9665620Z 
2019-10-02T17:39:57.9665640Z 
2019-10-02T17:39:57.9667255Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-02T17:39:57.9667652Z 
2019-10-02T17:39:57.9667761Z 
2019-10-02T17:39:57.9667818Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-02T17:39:57.9667892Z Build completed unsuccessfully in 0:59:27
2019-10-02T17:39:57.9667892Z Build completed unsuccessfully in 0:59:27
2019-10-02T17:39:57.9667944Z == clock drift check ==
2019-10-02T17:39:57.9668052Z   local time: Wed Oct  2 17:39:57 UTC 2019
2019-10-02T17:39:58.0495872Z   network time: Wed, 02 Oct 2019 17:39:58 GMT
2019-10-02T17:39:58.0500074Z == end clock drift check ==
2019-10-02T17:39:59.0100888Z ##[error]Bash exited with code '1'.
2019-10-02T17:39:59.0144341Z ##[section]Starting: Checkout
2019-10-02T17:39:59.0146162Z ==============================================================================
2019-10-02T17:39:59.0146391Z Task         : Get sources
2019-10-02T17:39:59.0146646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

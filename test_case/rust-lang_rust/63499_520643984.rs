plain
2019-08-12T23:24:07.4754209Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T23:24:07.4916516Z ##[command]git config gc.auto 0
2019-08-12T23:24:07.4987283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T23:24:07.5038700Z ##[command]git config --get-all http.proxy
2019-08-12T23:24:07.5159964Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63499/merge:refs/remotes/pull/63499/merge
---
2019-08-12T23:24:41.7970841Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T23:24:41.7970874Z 
2019-08-12T23:24:41.7971121Z   git checkout -b <new-branch-name>
2019-08-12T23:24:41.7971167Z 
2019-08-12T23:24:41.7971215Z HEAD is now at 0192f0517 Merge ad214fe47092c5b3d36a58480f6fa3f62d20770b into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-12T23:24:41.8115487Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T23:24:41.8117685Z ==============================================================================
2019-08-12T23:24:41.8117729Z Task         : Bash
2019-08-12T23:24:41.8117764Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T00:21:11.1989789Z .................................................................................................... 1400/8900
2019-08-13T00:21:17.1140899Z .................................................................................................... 1500/8900
2019-08-13T00:21:26.8188019Z ........................................................................................i........... 1600/8900
2019-08-13T00:21:33.3616892Z ....i............................................................................................... 1700/8900
2019-08-13T00:21:39.8599818Z ...............................................................................iiiii................ 1800/8900
2019-08-13T00:21:59.6713032Z .................................................................................................... 2000/8900
2019-08-13T00:22:01.7949069Z .................................................................................................... 2100/8900
2019-08-13T00:22:04.1353539Z .................................................................................................... 2200/8900
2019-08-13T00:22:11.0471006Z .................................................................................................... 2300/8900
---
2019-08-13T00:25:43.9624483Z .................................................................................................... 5300/8900
2019-08-13T00:25:50.7418070Z ............i....................................................................................... 5400/8900
2019-08-13T00:25:55.6712408Z .................................................................................................... 5500/8900
2019-08-13T00:26:07.0585175Z .................................................................................................... 5600/8900
2019-08-13T00:26:19.6871837Z .......ii...i..ii...........i....................................................................... 5700/8900
2019-08-13T00:26:34.8712391Z .................................................................................................... 5900/8900
2019-08-13T00:26:39.1333147Z .................................................................................................... 6000/8900
2019-08-13T00:26:39.1333147Z .................................................................................................... 6000/8900
2019-08-13T00:26:51.7763323Z ........i..ii....................................................................................... 6100/8900
2019-08-13T00:27:09.0027991Z ...................................................i................................................ 6300/8900
2019-08-13T00:27:10.8326543Z .................................................................................................... 6400/8900
2019-08-13T00:27:12.9411948Z .......................i............................................................................ 6500/8900
2019-08-13T00:27:16.9572132Z .................................................................................................... 6600/8900
---
2019-08-13T00:30:52.8129451Z - error[E0106]: missing lifetime specifier
2019-08-13T00:30:52.8129882Z + error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8130211Z 2   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:45
2019-08-13T00:30:52.8130262Z 3    |
2019-08-13T00:30:52.8130513Z 4 LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T00:30:52.8130767Z -    |                                             ^
2019-08-13T00:30:52.8130946Z -    |
2019-08-13T00:30:52.8130946Z -    |
2019-08-13T00:30:52.8131242Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T00:30:52.8131475Z +    |                          ----               ^^^^
2019-08-13T00:30:52.8131547Z +    |                          |                  |
2019-08-13T00:30:52.8131601Z +    |                          |                  ...but data from `f` is returned here
2019-08-13T00:30:52.8131660Z +    |                          this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8131969Z - error[E0106]: missing lifetime specifier
2019-08-13T00:30:52.8132210Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:60
2019-08-13T00:30:52.8132261Z + error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8132518Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:55
2019-08-13T00:30:52.8132518Z +   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:55
2019-08-13T00:30:52.8132565Z 11    |
2019-08-13T00:30:52.8132819Z 12 LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8133396Z -    |
2019-08-13T00:30:52.8133396Z -    |
2019-08-13T00:30:52.8133606Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T00:30:52.8133816Z +    |                          -----                        ^^^^^^^^^^^^^^^^^
2019-08-13T00:30:52.8134028Z +    |                          |                            |
2019-08-13T00:30:52.8134078Z +    |                          |                            ...but data from `f` is returned here
2019-08-13T00:30:52.8134143Z +    |                          this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8134382Z - error[E0106]: missing lifetime specifier
2019-08-13T00:30:52.8134586Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:67
2019-08-13T00:30:52.8134724Z -    |
2019-08-13T00:30:52.8134724Z -    |
2019-08-13T00:30:52.8134918Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8135265Z -    |
2019-08-13T00:30:52.8135265Z -    |
2019-08-13T00:30:52.8135475Z -    = note: return-position elided lifetimes require exactly one input-position elided lifetime, found multiple.
2019-08-13T00:30:52.8135806Z - error: cannot infer an appropriate lifetime
2019-08-13T00:30:52.8136168Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:33
2019-08-13T00:30:52.8136316Z -    |
2019-08-13T00:30:52.8136316Z -    |
2019-08-13T00:30:52.8136482Z - LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T00:30:52.8136683Z -    |                                 ^           ---- this return type evaluates to the `'static` lifetime...
2019-08-13T00:30:52.8137025Z -    |                                 ...but this borrow...
2019-08-13T00:30:52.8137155Z -    |
2019-08-13T00:30:52.8137155Z -    |
2019-08-13T00:30:52.8137332Z - note: ...can't outlive the lifetime '_ as defined on the method body at 10:26
2019-08-13T00:30:52.8137525Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:26
2019-08-13T00:30:52.8137656Z -    |
2019-08-13T00:30:52.8137819Z - LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T00:30:52.8137985Z -    |                          ^
2019-08-13T00:30:52.8138289Z - help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 10:26
2019-08-13T00:30:52.8138451Z -    |
2019-08-13T00:30:52.8138650Z - LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo + '_ { f }
2019-08-13T00:30:52.8138943Z - 
2019-08-13T00:30:52.8139113Z - error: cannot infer an appropriate lifetime
2019-08-13T00:30:52.8140151Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:16
2019-08-13T00:30:52.8140344Z -    |
2019-08-13T00:30:52.8140344Z -    |
2019-08-13T00:30:52.8140622Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8141168Z -    |                ^^^^ ...but this borrow...             ----------------- this return type evaluates to the `'static` lifetime...
2019-08-13T00:30:52.8141359Z -    |
2019-08-13T00:30:52.8141624Z - note: ...can't outlive the lifetime '_ as defined on the method body at 15:26
2019-08-13T00:30:52.8141887Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:26
2019-08-13T00:30:52.8142063Z -    |
2019-08-13T00:30:52.8142334Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8142541Z -    |                          ^
2019-08-13T00:30:52.8142857Z - help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 15:26
2019-08-13T00:30:52.8143212Z -    |
2019-08-13T00:30:52.8143768Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) + '_ { (self, f) }
2019-08-13T00:30:52.8144094Z - 
2019-08-13T00:30:52.8144248Z - error: cannot infer an appropriate lifetime
2019-08-13T00:30:52.8144610Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:34
2019-08-13T00:30:52.8144889Z -    |
2019-08-13T00:30:52.8144889Z -    |
2019-08-13T00:30:52.8145272Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8145513Z -    |                                  ^                    ----------------- this return type evaluates to the `'static` lifetime...
2019-08-13T00:30:52.8145888Z -    |                                  ...but this borrow...
2019-08-13T00:30:52.8146028Z -    |
2019-08-13T00:30:52.8146028Z -    |
2019-08-13T00:30:52.8146222Z - note: ...can't outlive the lifetime '_ as defined on the method body at 15:26
2019-08-13T00:30:52.8146433Z -   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:26
2019-08-13T00:30:52.8146574Z -    |
2019-08-13T00:30:52.8146773Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8146953Z -    |                          ^
2019-08-13T00:30:52.8147211Z - help: you can add a constraint to the return type to make it last less than `'static` and match the lifetime '_ as defined on the method body at 15:26
2019-08-13T00:30:52.8147366Z -    |
2019-08-13T00:30:52.8147589Z - LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) + '_ { (self, f) }
2019-08-13T00:30:52.8148126Z - 
2019-08-13T00:30:52.8148177Z 77 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8148530Z 78   --> $DIR/arbitrary_self_types_pin_lifetime_mismatch-async.rs:25:58
2019-08-13T00:30:52.8148564Z 79    |
2019-08-13T00:30:52.8148564Z 79    |
2019-08-13T00:30:52.8148600Z 
2019-08-13T00:30:52.8148638Z 83    |                                  |                       ...but data from `arg` is returned here
2019-08-13T00:30:52.8148681Z 84    |                                  this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8148897Z - error: aborting due to 7 previous errors
2019-08-13T00:30:52.8148933Z + error: aborting due to 3 previous errors
2019-08-13T00:30:52.8149026Z 87 
2019-08-13T00:30:52.8149237Z - For more information about this error, try `rustc --explain E0106`.
2019-08-13T00:30:52.8149237Z - For more information about this error, try `rustc --explain E0106`.
2019-08-13T00:30:52.8149272Z 89 
2019-08-13T00:30:52.8149292Z 
2019-08-13T00:30:52.8149311Z 
2019-08-13T00:30:52.8149364Z The actual stderr differed from the expected stderr.
2019-08-13T00:30:52.8150066Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async/arbitrary_self_types_pin_lifetime_mismatch-async.stderr
2019-08-13T00:30:52.8150321Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T00:30:52.8150622Z To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_mismatch-async.rs`
2019-08-13T00:30:52.8150703Z error: 1 errors occurred comparing output.
2019-08-13T00:30:52.8150768Z status: exit code: 1
2019-08-13T00:30:52.8150768Z status: exit code: 1
2019-08-13T00:30:52.8151612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async/auxiliary" "-A" "unused"
2019-08-13T00:30:52.8151943Z ------------------------------------------
2019-08-13T00:30:52.8151976Z 
2019-08-13T00:30:52.8152204Z ------------------------------------------
2019-08-13T00:30:52.8152249Z stderr:
2019-08-13T00:30:52.8152249Z stderr:
2019-08-13T00:30:52.8152455Z ------------------------------------------
2019-08-13T00:30:52.8152622Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8152909Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:10:45
2019-08-13T00:30:52.8152963Z    |
2019-08-13T00:30:52.8153530Z LL |     async fn a(self: Pin<&Foo>, f: &Foo) -> &Foo { f }
2019-08-13T00:30:52.8153868Z    |                          ----               ^^^^
2019-08-13T00:30:52.8153907Z    |                          |                  |
2019-08-13T00:30:52.8153949Z    |                          |                  ...but data from `f` is returned here
2019-08-13T00:30:52.8154011Z    |                          this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8154073Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8154296Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:15:55
2019-08-13T00:30:52.8154335Z    |
2019-08-13T00:30:52.8154335Z    |
2019-08-13T00:30:52.8154533Z LL |     async fn c(self: Pin<&Self>, f: &Foo, g: &Foo) -> (Pin<&Foo>, &Foo) { (self, f) }
2019-08-13T00:30:52.8154760Z    |                          -----                        ^^^^^^^^^^^^^^^^^
2019-08-13T00:30:52.8154803Z    |                          |                            |
2019-08-13T00:30:52.8154845Z    |                          |                            ...but data from `f` is returned here
2019-08-13T00:30:52.8154909Z    |                          this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8155143Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8155517Z   --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs:25:58
2019-08-13T00:30:52.8155748Z    |
2019-08-13T00:30:52.8155748Z    |
2019-08-13T00:30:52.8155945Z LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> &() { arg } //~ ERROR E0623
2019-08-13T00:30:52.8156127Z    |                                  -----                   ^^^
2019-08-13T00:30:52.8156184Z    |                                  |                       |
2019-08-13T00:30:52.8156292Z    |                                  |                       ...but data from `arg` is returned here
2019-08-13T00:30:52.8156343Z    |                                  this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8156420Z error: aborting due to 3 previous errors
2019-08-13T00:30:52.8156442Z 
2019-08-13T00:30:52.8156461Z 
2019-08-13T00:30:52.8156657Z ------------------------------------------
---
2019-08-13T00:30:52.8157073Z 10 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8157244Z -   --> $DIR/ref-mut-self-async.rs:24:52
2019-08-13T00:30:52.8157412Z +   --> $DIR/ref-mut-self-async.rs:21:52
2019-08-13T00:30:52.8157464Z 12    |
2019-08-13T00:30:52.8157645Z 13 LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-13T00:30:52.8157824Z 14    |                             ---------              ^^^^
2019-08-13T00:30:52.8157907Z 17    |                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8157944Z 18 
2019-08-13T00:30:52.8157978Z 19 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8158391Z -   --> $DIR/ref-mut-self-async.rs:28:61
2019-08-13T00:30:52.8158391Z -   --> $DIR/ref-mut-self-async.rs:28:61
2019-08-13T00:30:52.8158581Z +   --> $DIR/ref-mut-self-async.rs:25:61
2019-08-13T00:30:52.8158615Z 21    |
2019-08-13T00:30:52.8158996Z 22 LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8159175Z 23    |                                     ---------               ^^^^
2019-08-13T00:30:52.8159327Z 26    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8159382Z 27 
2019-08-13T00:30:52.8159416Z 28 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8159594Z -   --> $DIR/ref-mut-self-async.rs:32:61
2019-08-13T00:30:52.8159594Z -   --> $DIR/ref-mut-self-async.rs:32:61
2019-08-13T00:30:52.8160168Z +   --> $DIR/ref-mut-self-async.rs:29:61
2019-08-13T00:30:52.8160220Z 30    |
2019-08-13T00:30:52.8160465Z 31 LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8160728Z 32    |                                     ---------               ^^^^
2019-08-13T00:30:52.8160817Z 35    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8160883Z 36 
2019-08-13T00:30:52.8160927Z 37 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8161137Z -   --> $DIR/ref-mut-self-async.rs:36:70
2019-08-13T00:30:52.8161137Z -   --> $DIR/ref-mut-self-async.rs:36:70
2019-08-13T00:30:52.8161363Z +   --> $DIR/ref-mut-self-async.rs:33:70
2019-08-13T00:30:52.8161420Z 39    |
2019-08-13T00:30:52.8161680Z 40 LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8161933Z 41    |                                             ---------                ^^^^
2019-08-13T00:30:52.8162039Z 44    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8162089Z 45 
2019-08-13T00:30:52.8162155Z 46 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8162365Z -   --> $DIR/ref-mut-self-async.rs:40:70
2019-08-13T00:30:52.8162365Z -   --> $DIR/ref-mut-self-async.rs:40:70
2019-08-13T00:30:52.8162574Z +   --> $DIR/ref-mut-self-async.rs:37:70
2019-08-13T00:30:52.8162635Z 48    |
2019-08-13T00:30:52.8162884Z 49 LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8163133Z 50    |                                             ---------                ^^^^
2019-08-13T00:30:52.8163219Z 
2019-08-13T00:30:52.8163264Z The actual stderr differed from the expected stderr.
2019-08-13T00:30:52.8164288Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async/ref-mut-self-async.stderr
2019-08-13T00:30:52.8164288Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async/ref-mut-self-async.stderr
2019-08-13T00:30:52.8164507Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T00:30:52.8164697Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self-async.rs`
2019-08-13T00:30:52.8164754Z error: 1 errors occurred comparing output.
2019-08-13T00:30:52.8164803Z status: exit code: 1
2019-08-13T00:30:52.8164803Z status: exit code: 1
2019-08-13T00:30:52.8165358Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self-async/auxiliary" "-A" "unused"
2019-08-13T00:30:52.8165604Z ------------------------------------------
2019-08-13T00:30:52.8165627Z 
2019-08-13T00:30:52.8165798Z ------------------------------------------
2019-08-13T00:30:52.8165831Z stderr:
2019-08-13T00:30:52.8165831Z stderr:
2019-08-13T00:30:52.8166160Z ------------------------------------------
2019-08-13T00:30:52.8166213Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8166397Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:15:46
2019-08-13T00:30:52.8166436Z    |
2019-08-13T00:30:52.8166647Z LL |     async fn ref_self(&mut self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-13T00:30:52.8167208Z    |                       ---------              ^^^^
2019-08-13T00:30:52.8167251Z    |                       |                      |
2019-08-13T00:30:52.8167403Z    |                       |                      ...but data from `f` is returned here
2019-08-13T00:30:52.8167449Z    |                       this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8167513Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8167743Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:21:52
2019-08-13T00:30:52.8167780Z    |
2019-08-13T00:30:52.8167780Z    |
2019-08-13T00:30:52.8167956Z LL |     async fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-08-13T00:30:52.8168250Z    |                             ---------              ^^^^
2019-08-13T00:30:52.8168290Z    |                             |                      |
2019-08-13T00:30:52.8168331Z    |                             |                      ...but data from `f` is returned here
2019-08-13T00:30:52.8168563Z    |                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8168631Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8168847Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:25:61
2019-08-13T00:30:52.8168889Z    |
2019-08-13T00:30:52.8168889Z    |
2019-08-13T00:30:52.8169078Z LL |     async fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8169526Z    |                                     ---------               ^^^^
2019-08-13T00:30:52.8169567Z    |                                     |                       |
2019-08-13T00:30:52.8169611Z    |                                     |                       ...but data from `f` is returned here
2019-08-13T00:30:52.8169821Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8170078Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8170367Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:29:61
2019-08-13T00:30:52.8170434Z    |
2019-08-13T00:30:52.8170434Z    |
2019-08-13T00:30:52.8170683Z LL |     async fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8171024Z    |                                     ---------               ^^^^
2019-08-13T00:30:52.8171104Z    |                                     |                       |
2019-08-13T00:30:52.8171160Z    |                                     |                       ...but data from `f` is returned here
2019-08-13T00:30:52.8171221Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8171317Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8171577Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:33:70
2019-08-13T00:30:52.8171687Z    |
2019-08-13T00:30:52.8171687Z    |
2019-08-13T00:30:52.8171940Z LL |     async fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8172185Z    |                                             ---------                ^^^^
2019-08-13T00:30:52.8172249Z    |                                             |                        |
2019-08-13T00:30:52.8172331Z    |                                             |                        ...but data from `f` is returned here
2019-08-13T00:30:52.8172394Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8172489Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8172729Z   --> /checkout/src/test/ui/self/elision/ref-mut-self-async.rs:37:70
2019-08-13T00:30:52.8172777Z    |
2019-08-13T00:30:52.8172777Z    |
2019-08-13T00:30:52.8173039Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8173287Z    |                                             ---------                ^^^^
2019-08-13T00:30:52.8173342Z    |                                             |                        |
2019-08-13T00:30:52.8173416Z    |                                             |                        ...but data from `f` is returned here
2019-08-13T00:30:52.8173721Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8173793Z error: aborting due to 6 previous errors
2019-08-13T00:30:52.8173814Z 
2019-08-13T00:30:52.8173833Z 
2019-08-13T00:30:52.8174002Z ------------------------------------------
---
2019-08-13T00:30:52.8174395Z 10 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8174567Z -   --> $DIR/ref-self-async.rs:33:48
2019-08-13T00:30:52.8174718Z +   --> $DIR/ref-self-async.rs:30:48
2019-08-13T00:30:52.8174757Z 12    |
2019-08-13T00:30:52.8174925Z 13 LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T00:30:52.8175112Z 14    |                             -----              ^^^^
2019-08-13T00:30:52.8175176Z 17    |                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8175228Z 18 
2019-08-13T00:30:52.8175259Z 19 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8175411Z -   --> $DIR/ref-self-async.rs:37:57
2019-08-13T00:30:52.8175411Z -   --> $DIR/ref-self-async.rs:37:57
2019-08-13T00:30:52.8175577Z +   --> $DIR/ref-self-async.rs:34:57
2019-08-13T00:30:52.8175609Z 21    |
2019-08-13T00:30:52.8175784Z 22 LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8175957Z 23    |                                     -----               ^^^^
2019-08-13T00:30:52.8176036Z 26    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8176078Z 27 
2019-08-13T00:30:52.8176126Z 28 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8176331Z -   --> $DIR/ref-self-async.rs:41:57
2019-08-13T00:30:52.8176331Z -   --> $DIR/ref-self-async.rs:41:57
2019-08-13T00:30:52.8176503Z +   --> $DIR/ref-self-async.rs:38:57
2019-08-13T00:30:52.8176551Z 30    |
2019-08-13T00:30:52.8176726Z 31 LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8176899Z 32    |                                     -----               ^^^^
2019-08-13T00:30:52.8176978Z 35    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8177013Z 36 
2019-08-13T00:30:52.8177043Z 37 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8177214Z -   --> $DIR/ref-self-async.rs:45:66
2019-08-13T00:30:52.8177214Z -   --> $DIR/ref-self-async.rs:45:66
2019-08-13T00:30:52.8177366Z +   --> $DIR/ref-self-async.rs:42:66
2019-08-13T00:30:52.8177398Z 39    |
2019-08-13T00:30:52.8177594Z 40 LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8177785Z 41    |                                             -----                ^^^^
2019-08-13T00:30:52.8177854Z 44    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8177906Z 45 
2019-08-13T00:30:52.8177937Z 46 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8178090Z -   --> $DIR/ref-self-async.rs:49:66
2019-08-13T00:30:52.8178090Z -   --> $DIR/ref-self-async.rs:49:66
2019-08-13T00:30:52.8178257Z +   --> $DIR/ref-self-async.rs:46:66
2019-08-13T00:30:52.8178289Z 48    |
2019-08-13T00:30:52.8178467Z 49 LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8178663Z 50    |                                             -----                ^^^^
2019-08-13T00:30:52.8178726Z 53    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8178761Z 54 
2019-08-13T00:30:52.8178808Z 55 error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8179049Z -   --> $DIR/ref-self-async.rs:53:69
2019-08-13T00:30:52.8179049Z -   --> $DIR/ref-self-async.rs:53:69
2019-08-13T00:30:52.8179206Z +   --> $DIR/ref-self-async.rs:50:69
2019-08-13T00:30:52.8179255Z 57    |
2019-08-13T00:30:52.8179436Z 58 LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-08-13T00:30:52.8180301Z 59    |                                            -----                    ^^^
2019-08-13T00:30:52.8180395Z 
2019-08-13T00:30:52.8180441Z The actual stderr differed from the expected stderr.
2019-08-13T00:30:52.8180739Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async/ref-self-async.stderr
2019-08-13T00:30:52.8180739Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async/ref-self-async.stderr
2019-08-13T00:30:52.8180997Z To update references, rerun the tests and pass the `--bless` flag
2019-08-13T00:30:52.8181292Z To only update this specific test, also pass `--test-args self/elision/ref-self-async.rs`
2019-08-13T00:30:52.8181396Z error: 1 errors occurred comparing output.
2019-08-13T00:30:52.8181456Z status: exit code: 1
2019-08-13T00:30:52.8181456Z status: exit code: 1
2019-08-13T00:30:52.8182239Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self-async/auxiliary" "-A" "unused"
2019-08-13T00:30:52.8182591Z ------------------------------------------
2019-08-13T00:30:52.8182628Z 
2019-08-13T00:30:52.8182875Z ------------------------------------------
2019-08-13T00:30:52.8182922Z stderr:
2019-08-13T00:30:52.8182922Z stderr:
2019-08-13T00:30:52.8183146Z ------------------------------------------
2019-08-13T00:30:52.8183214Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8183587Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:24:42
2019-08-13T00:30:52.8183704Z    |
2019-08-13T00:30:52.8183930Z LL |     async fn ref_self(&self, f: &u32) -> &u32 { //~ ERROR lifetime mismatch
2019-08-13T00:30:52.8184093Z    |                       -----              ^^^^
2019-08-13T00:30:52.8184130Z    |                       |                  |
2019-08-13T00:30:52.8184183Z    |                       |                  ...but data from `f` is returned here
2019-08-13T00:30:52.8184225Z    |                       this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8184300Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8184471Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:30:48
2019-08-13T00:30:52.8184505Z    |
2019-08-13T00:30:52.8184505Z    |
2019-08-13T00:30:52.8184667Z LL |     async fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-08-13T00:30:52.8184848Z    |                             -----              ^^^^
2019-08-13T00:30:52.8184893Z    |                             |                  |
2019-08-13T00:30:52.8184937Z    |                             |                  ...but data from `f` is returned here
2019-08-13T00:30:52.8184997Z    |                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8185053Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8185242Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:34:57
2019-08-13T00:30:52.8185276Z    |
2019-08-13T00:30:52.8185276Z    |
2019-08-13T00:30:52.8185445Z LL |     async fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8185629Z    |                                     -----               ^^^^
2019-08-13T00:30:52.8185667Z    |                                     |                   |
2019-08-13T00:30:52.8185707Z    |                                     |                   ...but data from `f` is returned here
2019-08-13T00:30:52.8185832Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8185887Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8186073Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:38:57
2019-08-13T00:30:52.8186124Z    |
2019-08-13T00:30:52.8186124Z    |
2019-08-13T00:30:52.8186294Z LL |     async fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8186465Z    |                                     -----               ^^^^
2019-08-13T00:30:52.8186519Z    |                                     |                   |
2019-08-13T00:30:52.8186559Z    |                                     |                   ...but data from `f` is returned here
2019-08-13T00:30:52.8186603Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8186675Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8186855Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:42:66
2019-08-13T00:30:52.8186905Z    |
2019-08-13T00:30:52.8186905Z    |
2019-08-13T00:30:52.8187087Z LL |     async fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8187266Z    |                                             -----                ^^^^
2019-08-13T00:30:52.8187306Z    |                                             |                    |
2019-08-13T00:30:52.8187362Z    |                                             |                    ...but data from `f` is returned here
2019-08-13T00:30:52.8187407Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8187480Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8187652Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:46:66
2019-08-13T00:30:52.8187685Z    |
2019-08-13T00:30:52.8187685Z    |
2019-08-13T00:30:52.8187876Z LL |     async fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-08-13T00:30:52.8188062Z    |                                             -----                ^^^^
2019-08-13T00:30:52.8188155Z    |                                             |                    |
2019-08-13T00:30:52.8188217Z    |                                             |                    ...but data from `f` is returned here
2019-08-13T00:30:52.8188262Z    |                                             this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8188332Z error[E0623]: lifetime mismatch
2019-08-13T00:30:52.8188519Z   --> /checkout/src/test/ui/self/elision/ref-self-async.rs:50:69
2019-08-13T00:30:52.8188554Z    |
2019-08-13T00:30:52.8188554Z    |
2019-08-13T00:30:52.8188729Z LL |     async fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-08-13T00:30:52.8188924Z    |                                            -----                    ^^^
2019-08-13T00:30:52.8188963Z    |                                            |                        |
2019-08-13T00:30:52.8189016Z    |                                            |                        ...but data from `f` is returned here
2019-08-13T00:30:52.8189077Z    |                                            this parameter and the return type are declared with different lifetimes...
2019-08-13T00:30:52.8189134Z error: aborting due to 7 previous errors
2019-08-13T00:30:52.8189172Z 
2019-08-13T00:30:52.8189190Z 
2019-08-13T00:30:52.8189344Z ------------------------------------------
2019-08-13T00:30:52.8189344Z ------------------------------------------
2019-08-13T00:30:52.8189367Z 
2019-08-13T00:30:52.8189386Z 
2019-08-13T00:30:52.8189941Z ---- [ui] ui/self/self_lifetime-async.rs stdout ----
2019-08-13T00:30:52.8189979Z 
2019-08-13T00:30:52.8190024Z error: ui test compiled successfully!
2019-08-13T00:30:52.8190068Z status: exit code: 0
2019-08-13T00:30:52.8190861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_lifetime-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async/auxiliary" "-A" "unused"
2019-08-13T00:30:52.8191303Z ------------------------------------------
2019-08-13T00:30:52.8191337Z 
2019-08-13T00:30:52.8191544Z ------------------------------------------
2019-08-13T00:30:52.8191607Z stderr:
---
2019-08-13T00:30:52.8193669Z test result: FAILED. 8863 passed; 4 failed; 33 ignored; 0 measured; 0 filtered out
2019-08-13T00:30:52.8193696Z 
2019-08-13T00:30:52.8198089Z 
2019-08-13T00:30:52.8198145Z 
2019-08-13T00:30:52.8199452Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-13T00:30:52.8200400Z 
2019-08-13T00:30:52.8200433Z 
2019-08-13T00:30:52.8200482Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-13T00:30:52.8200569Z Build completed unsuccessfully in 1:00:33
2019-08-13T00:30:52.8200569Z Build completed unsuccessfully in 1:00:33
2019-08-13T00:30:52.8200952Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-13T00:30:52.8201034Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-13T00:30:53.5931023Z ##[error]Bash exited with code '1'.
2019-08-13T00:30:53.5999656Z ##[section]Starting: Checkout
2019-08-13T00:30:53.6001949Z ==============================================================================
2019-08-13T00:30:53.6002006Z Task         : Get sources
2019-08-13T00:30:53.6002071Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

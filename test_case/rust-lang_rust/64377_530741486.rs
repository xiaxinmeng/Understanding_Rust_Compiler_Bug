plain
2019-09-12T08:00:14.2123941Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T08:00:14.2411985Z ##[command]git config gc.auto 0
2019-09-12T08:00:14.2458560Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T08:00:14.2512559Z ##[command]git config --get-all http.proxy
2019-09-12T08:00:14.2652357Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64377/merge:refs/remotes/pull/64377/merge
---
2019-09-12T09:08:18.8665956Z .................................................................................................... 1500/9010
2019-09-12T09:08:24.9426810Z ...................................................................F..........F..................... 1600/9010
2019-09-12T09:08:38.5544179Z .........................................................i...............i.......................... 1700/9010
2019-09-12T09:08:47.1016893Z .................................................................................................... 1800/9010
2019-09-12T09:09:03.3501411Z ................................................iiiii............................................... 1900/9010
2019-09-12T09:09:15.4396418Z .................................................................................................... 2100/9010
2019-09-12T09:09:18.2515778Z .................................................................................................... 2200/9010
2019-09-12T09:09:22.3456203Z .................................................................................................... 2300/9010
2019-09-12T09:09:31.5628909Z .................................................................................................... 2400/9010
---
2019-09-12T09:12:47.3213618Z ...................................i...............i................................................ 4700/9010
2019-09-12T09:12:59.6365360Z .................................................................................................... 4800/9010
2019-09-12T09:13:07.0235019Z .................................................................................................... 4900/9010
2019-09-12T09:13:18.6542895Z .................................................................................................... 5000/9010
2019-09-12T09:13:25.4228068Z ..................ii.ii............................................................................. 5100/9010
2019-09-12T09:13:36.5741943Z .................................................................................................... 5300/9010
2019-09-12T09:13:47.5764450Z .................................................................................i.................. 5400/9010
2019-09-12T09:13:56.2470712Z .................................................................................................... 5500/9010
2019-09-12T09:14:02.4845949Z .................................................................................................... 5600/9010
2019-09-12T09:14:02.4845949Z .................................................................................................... 5600/9010
2019-09-12T09:14:14.1253801Z ............................................................................ii...i..ii...........i.. 5700/9010
2019-09-12T09:14:41.5345818Z .................................................................................................... 5900/9010
2019-09-12T09:14:52.1079006Z .................................................................................................... 6000/9010
2019-09-12T09:14:52.1079006Z .................................................................................................... 6000/9010
2019-09-12T09:15:01.7416287Z ..............................................................................i..ii................. 6100/9010
2019-09-12T09:15:34.3929524Z .................................................................................................... 6300/9010
2019-09-12T09:15:36.7692462Z .....................................i.............................................................. 6400/9010
2019-09-12T09:15:39.1196348Z .................................................................................................... 6500/9010
2019-09-12T09:15:41.9607654Z .........i.......................................................................................... 6600/9010
---
2019-09-12T09:20:08.9651073Z 
2019-09-12T09:20:08.9651190Z 
2019-09-12T09:20:08.9651568Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9652278Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/check-static-values-constraints.stderr
2019-09-12T09:20:08.9652739Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9653166Z To only update this specific test, also pass `--test-args check-static-values-constraints.rs`
2019-09-12T09:20:08.9653529Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9653675Z status: exit code: 1
2019-09-12T09:20:08.9653675Z status: exit code: 1
2019-09-12T09:20:08.9654592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9658294Z ------------------------------------------
2019-09-12T09:20:08.9660415Z 
2019-09-12T09:20:08.9662219Z ------------------------------------------
2019-09-12T09:20:08.9662519Z stderr:
2019-09-12T09:20:08.9662519Z stderr:
2019-09-12T09:20:08.9662902Z ------------------------------------------
2019-09-12T09:20:08.9663306Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9664182Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:65:43
2019-09-12T09:20:08.9664514Z    |
2019-09-12T09:20:08.9664675Z LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
2019-09-12T09:20:08.9664867Z    |  ___________________________________________^
2019-09-12T09:20:08.9665273Z LL | | //~^ ERROR destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9665481Z LL | |                                                      field2: SafeEnum::Variant1}};
2019-09-12T09:20:08.9665641Z    | |________________________________________________________________________________^ statics cannot evaluate destructors
2019-09-12T09:20:08.9665970Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9666353Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
2019-09-12T09:20:08.9666524Z    |
2019-09-12T09:20:08.9666524Z    |
2019-09-12T09:20:08.9666691Z LL | static STATIC11: Box<MyOwned> = box MyOwned;
2019-09-12T09:20:08.9667352Z    |                                 ^^^^^^^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9667891Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9668490Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:79:37
2019-09-12T09:20:08.9668677Z    |
2019-09-12T09:20:08.9668677Z    |
2019-09-12T09:20:08.9668819Z LL | static STATIC11: Box<MyOwned> = box MyOwned;
2019-09-12T09:20:08.9669091Z 
2019-09-12T09:20:08.9669235Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-12T09:20:08.9670176Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:90:32
2019-09-12T09:20:08.9670401Z    |
2019-09-12T09:20:08.9670401Z    |
2019-09-12T09:20:08.9670703Z LL |     field2: SafeEnum::Variant4("str".to_string())
2019-09-12T09:20:08.9670987Z 
2019-09-12T09:20:08.9671123Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9671512Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
2019-09-12T09:20:08.9671700Z    |
2019-09-12T09:20:08.9671700Z    |
2019-09-12T09:20:08.9671842Z LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9671982Z    |     ^^^^^^^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9672250Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9682145Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:95:9
2019-09-12T09:20:08.9686689Z    |
2019-09-12T09:20:08.9686689Z    |
2019-09-12T09:20:08.9687053Z LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9687174Z 
2019-09-12T09:20:08.9687221Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9687682Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:97:5
2019-09-12T09:20:08.9687735Z    |
2019-09-12T09:20:08.9687735Z    |
2019-09-12T09:20:08.9687783Z LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9687852Z    |     ^^^^^^^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9687930Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9688186Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:97:9
2019-09-12T09:20:08.9688250Z    |
2019-09-12T09:20:08.9688250Z    |
2019-09-12T09:20:08.9688299Z LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9688375Z 
2019-09-12T09:20:08.9688449Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9688709Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:102:6
2019-09-12T09:20:08.9688758Z    |
2019-09-12T09:20:08.9688758Z    |
2019-09-12T09:20:08.9688822Z LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9688873Z    |      ^^^^^^^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9688950Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9689223Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:102:10
2019-09-12T09:20:08.9689272Z    |
2019-09-12T09:20:08.9689272Z    |
2019-09-12T09:20:08.9689320Z LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9689413Z 
2019-09-12T09:20:08.9689458Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9689725Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:104:6
2019-09-12T09:20:08.9697536Z    |
2019-09-12T09:20:08.9697536Z    |
2019-09-12T09:20:08.9697638Z LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9697696Z    |      ^^^^^^^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9697816Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9698372Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:104:10
2019-09-12T09:20:08.9698451Z    |
2019-09-12T09:20:08.9698451Z    |
2019-09-12T09:20:08.9698502Z LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
2019-09-12T09:20:08.9698580Z 
2019-09-12T09:20:08.9698647Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9698920Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:111:5
2019-09-12T09:20:08.9698969Z    |
---
2019-09-12T09:20:08.9700014Z 
2019-09-12T09:20:08.9700059Z error[E0507]: cannot move out of static item `x`
2019-09-12T09:20:08.9700347Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:116:45
2019-09-12T09:20:08.9700416Z    |
2019-09-12T09:20:08.9700463Z LL |     let y = { static x: Box<isize> = box 3; x };
2019-09-12T09:20:08.9700581Z    |                                             |
2019-09-12T09:20:08.9700581Z    |                                             |
2019-09-12T09:20:08.9700643Z    |                                             move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
2019-09-12T09:20:08.9700704Z    |                                             help: consider borrowing here: `&x`
2019-09-12T09:20:08.9700821Z error[E0010]: allocations are not allowed in statics
2019-09-12T09:20:08.9701082Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:116:38
2019-09-12T09:20:08.9701146Z    |
2019-09-12T09:20:08.9701146Z    |
2019-09-12T09:20:08.9701191Z LL |     let y = { static x: Box<isize> = box 3; x };
2019-09-12T09:20:08.9702115Z    |                                      ^^^^^ allocation not allowed in statics
2019-09-12T09:20:08.9702250Z error[E0019]: static contains unimplemented expression type
2019-09-12T09:20:08.9702616Z   --> /checkout/src/test/ui/check-static-values-constraints.rs:116:42
2019-09-12T09:20:08.9702667Z    |
2019-09-12T09:20:08.9702667Z    |
2019-09-12T09:20:08.9702730Z LL |     let y = { static x: Box<isize> = box 3; x };
2019-09-12T09:20:08.9702809Z 
2019-09-12T09:20:08.9702854Z error: aborting due to 17 previous errors
2019-09-12T09:20:08.9702916Z 
2019-09-12T09:20:08.9702964Z Some errors have detailed explanations: E0010, E0015, E0019, E0493, E0507.
---
2019-09-12T09:20:08.9704613Z 27 
2019-09-12T09:20:08.9704657Z 
2019-09-12T09:20:08.9704683Z 
2019-09-12T09:20:08.9704728Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9705028Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/const_let.stderr
2019-09-12T09:20:08.9705316Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9706115Z To only update this specific test, also pass `--test-args consts/const-eval/const_let.rs`
2019-09-12T09:20:08.9706224Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9706270Z status: exit code: 1
2019-09-12T09:20:08.9706270Z status: exit code: 1
2019-09-12T09:20:08.9707530Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9708447Z ------------------------------------------
2019-09-12T09:20:08.9708492Z 
2019-09-12T09:20:08.9708766Z ------------------------------------------
2019-09-12T09:20:08.9708814Z stderr:
2019-09-12T09:20:08.9708814Z stderr:
2019-09-12T09:20:08.9709027Z ------------------------------------------
2019-09-12T09:20:08.9709289Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9709534Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:16:32
2019-09-12T09:20:08.9709586Z    |
2019-09-12T09:20:08.9709653Z LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2019-09-12T09:20:08.9709711Z    |                                ^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9711423Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9714240Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:20:33
2019-09-12T09:20:08.9714313Z    |
2019-09-12T09:20:08.9714313Z    |
2019-09-12T09:20:08.9714404Z LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2019-09-12T09:20:08.9714469Z    |                                 ^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9714851Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9720638Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:24:21
2019-09-12T09:20:08.9720724Z    |
2019-09-12T09:20:08.9720724Z    |
2019-09-12T09:20:08.9720775Z LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
2019-09-12T09:20:08.9720859Z    |                     ^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9721305Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9721557Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:28:22
2019-09-12T09:20:08.9721628Z    |
2019-09-12T09:20:08.9721628Z    |
2019-09-12T09:20:08.9721678Z LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2019-09-12T09:20:08.9721749Z    |                      ^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9721853Z error: aborting due to 4 previous errors
2019-09-12T09:20:08.9721883Z 
2019-09-12T09:20:08.9722133Z For more information about this error, try `rustc --explain E0493`.
2019-09-12T09:20:08.9722186Z 
---
2019-09-12T09:20:08.9724137Z 
2019-09-12T09:20:08.9724163Z 
2019-09-12T09:20:08.9724209Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9762396Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-09-12T09:20:08.9763091Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9763648Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-09-12T09:20:08.9763767Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9763814Z status: exit code: 1
2019-09-12T09:20:08.9763814Z status: exit code: 1
2019-09-12T09:20:08.9764813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9765295Z ------------------------------------------
2019-09-12T09:20:08.9765332Z 
2019-09-12T09:20:08.9765558Z ------------------------------------------
2019-09-12T09:20:08.9765605Z stderr:
2019-09-12T09:20:08.9765605Z stderr:
2019-09-12T09:20:08.9765835Z ------------------------------------------
2019-09-12T09:20:08.9766074Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9766326Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
2019-09-12T09:20:08.9766391Z    |
2019-09-12T09:20:08.9766649Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-09-12T09:20:08.9766729Z    |                         ^^^^ constant functions cannot evaluate destructors
2019-09-12T09:20:08.9766823Z error[E0723]: mutable references in const fn are unstable
2019-09-12T09:20:08.9767076Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
2019-09-12T09:20:08.9767132Z    |
2019-09-12T09:20:08.9767132Z    |
2019-09-12T09:20:08.9767368Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-09-12T09:20:08.9767476Z    |
2019-09-12T09:20:08.9767476Z    |
2019-09-12T09:20:08.9767900Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9767961Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9768272Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9768522Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
2019-09-12T09:20:08.9768581Z    |
2019-09-12T09:20:08.9768581Z    |
2019-09-12T09:20:08.9768874Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-09-12T09:20:08.9768935Z    |                            ^^^^ constant functions cannot evaluate destructors
2019-09-12T09:20:08.9769030Z error[E0723]: mutable references in const fn are unstable
2019-09-12T09:20:08.9769280Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
2019-09-12T09:20:08.9769329Z    |
2019-09-12T09:20:08.9769329Z    |
2019-09-12T09:20:08.9769585Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-09-12T09:20:08.9769686Z    |
2019-09-12T09:20:08.9769686Z    |
2019-09-12T09:20:08.9770145Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9770201Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9770523Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9770804Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
2019-09-12T09:20:08.9770850Z    |
2019-09-12T09:20:08.9770850Z    |
2019-09-12T09:20:08.9771090Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-09-12T09:20:08.9771157Z    |                           ^^^^ constant functions cannot evaluate destructors
2019-09-12T09:20:08.9771597Z error[E0723]: mutable references in const fn are unstable
2019-09-12T09:20:08.9771970Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
2019-09-12T09:20:08.9772020Z    |
2019-09-12T09:20:08.9772020Z    |
2019-09-12T09:20:08.9772255Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-09-12T09:20:08.9772357Z    |
2019-09-12T09:20:08.9772357Z    |
2019-09-12T09:20:08.9772657Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9772723Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9773029Z error[E0723]: mutable references in const fn are unstable
2019-09-12T09:20:08.9773625Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
2019-09-12T09:20:08.9773686Z    |
2019-09-12T09:20:08.9773686Z    |
2019-09-12T09:20:08.9773936Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-09-12T09:20:08.9774040Z    |
2019-09-12T09:20:08.9774040Z    |
2019-09-12T09:20:08.9774454Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9774514Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9774558Z 
2019-09-12T09:20:08.9774608Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9774939Z    |
2019-09-12T09:20:08.9774939Z    |
2019-09-12T09:20:08.9775173Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-09-12T09:20:08.9775288Z    |
2019-09-12T09:20:08.9775288Z    |
2019-09-12T09:20:08.9775586Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9775643Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9775692Z 
2019-09-12T09:20:08.9775740Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9776052Z    |
2019-09-12T09:20:08.9776052Z    |
2019-09-12T09:20:08.9776292Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-09-12T09:20:08.9776382Z    |
2019-09-12T09:20:08.9776382Z    |
2019-09-12T09:20:08.9776679Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9776736Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9776768Z 
2019-09-12T09:20:08.9776833Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-12T09:20:08.9777165Z    |
2019-09-12T09:20:08.9777165Z    |
2019-09-12T09:20:08.9777411Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-09-12T09:20:08.9777506Z    |
2019-09-12T09:20:08.9777506Z    |
2019-09-12T09:20:08.9777803Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9777859Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9777892Z 
2019-09-12T09:20:08.9777939Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-12T09:20:08.9778262Z    |
2019-09-12T09:20:08.9778262Z    |
2019-09-12T09:20:08.9778486Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-09-12T09:20:08.9778595Z    |
2019-09-12T09:20:08.9778595Z    |
2019-09-12T09:20:08.9778873Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9778964Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9779042Z error[E0723]: only int and `bool` operations are stable in const fn
2019-09-12T09:20:08.9779318Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
2019-09-12T09:20:08.9779366Z    |
2019-09-12T09:20:08.9779366Z    |
2019-09-12T09:20:08.9779588Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-09-12T09:20:08.9779698Z    |
2019-09-12T09:20:08.9779698Z    |
2019-09-12T09:20:08.9779974Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9780045Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9780077Z 
2019-09-12T09:20:08.9780124Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-12T09:20:08.9780566Z    |
2019-09-12T09:20:08.9780566Z    |
2019-09-12T09:20:08.9780836Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-09-12T09:20:08.9781039Z    |
2019-09-12T09:20:08.9781039Z    |
2019-09-12T09:20:08.9781362Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9781419Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9781471Z 
2019-09-12T09:20:08.9781516Z error[E0723]: cannot access `static` items in const fn
2019-09-12T09:20:08.9781892Z    |
2019-09-12T09:20:08.9781892Z    |
2019-09-12T09:20:08.9782153Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-09-12T09:20:08.9782265Z    |
2019-09-12T09:20:08.9782265Z    |
2019-09-12T09:20:08.9782545Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9782601Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9782658Z 
2019-09-12T09:20:08.9782712Z error[E0723]: cannot access `static` items in const fn
2019-09-12T09:20:08.9783019Z    |
2019-09-12T09:20:08.9783019Z    |
2019-09-12T09:20:08.9783296Z LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
2019-09-12T09:20:08.9783393Z    |
2019-09-12T09:20:08.9783393Z    |
2019-09-12T09:20:08.9783809Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9783869Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9783961Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-12T09:20:08.9784226Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
2019-09-12T09:20:08.9784272Z    |
2019-09-12T09:20:08.9784272Z    |
2019-09-12T09:20:08.9784520Z LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-09-12T09:20:08.9784635Z    |
2019-09-12T09:20:08.9784635Z    |
2019-09-12T09:20:08.9784930Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9784986Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9785079Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-12T09:20:08.9785333Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
2019-09-12T09:20:08.9785380Z    |
2019-09-12T09:20:08.9785380Z    |
2019-09-12T09:20:08.9785632Z LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-09-12T09:20:08.9785751Z    |
2019-09-12T09:20:08.9785751Z    |
2019-09-12T09:20:08.9786043Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9786099Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9786195Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-12T09:20:08.9786997Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
2019-09-12T09:20:08.9787076Z    |
2019-09-12T09:20:08.9787076Z    |
2019-09-12T09:20:08.9787334Z LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-09-12T09:20:08.9787453Z    |
2019-09-12T09:20:08.9787453Z    |
2019-09-12T09:20:08.9787798Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9787876Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9787961Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-12T09:20:08.9788400Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
2019-09-12T09:20:08.9788449Z    |
2019-09-12T09:20:08.9788449Z    |
2019-09-12T09:20:08.9788697Z LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-09-12T09:20:08.9789108Z    |
2019-09-12T09:20:08.9789108Z    |
2019-09-12T09:20:08.9789455Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9789531Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9789610Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-12T09:20:08.9789865Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:100:38
2019-09-12T09:20:08.9789931Z    |
2019-09-12T09:20:08.9789931Z    |
2019-09-12T09:20:08.9790167Z LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
2019-09-12T09:20:08.9790277Z    |
2019-09-12T09:20:08.9790277Z    |
2019-09-12T09:20:08.9790549Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9790617Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9790706Z error[E0723]: loops are not allowed in const fn
2019-09-12T09:20:08.9791367Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:102:29
2019-09-12T09:20:08.9791456Z    |
2019-09-12T09:20:08.9791456Z    |
2019-09-12T09:20:08.9791501Z LL | const fn foo30_5(b: bool) { while b { } }
2019-09-12T09:20:08.9791606Z    |
2019-09-12T09:20:08.9791606Z    |
2019-09-12T09:20:08.9791969Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9792028Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9792120Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-12T09:20:08.9792594Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:105:44
2019-09-12T09:20:08.9792643Z    |
2019-09-12T09:20:08.9792643Z    |
2019-09-12T09:20:08.9793049Z LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-09-12T09:20:08.9793154Z    |
2019-09-12T09:20:08.9793154Z    |
2019-09-12T09:20:08.9793515Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9793572Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9793666Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-12T09:20:08.9794612Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:107:44
2019-09-12T09:20:08.9794670Z    |
2019-09-12T09:20:08.9794670Z    |
2019-09-12T09:20:08.9794930Z LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-09-12T09:20:08.9795026Z    |
2019-09-12T09:20:08.9795026Z    |
2019-09-12T09:20:08.9795332Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9795586Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9795720Z error[E0723]: mutable references in const fn are unstable
2019-09-12T09:20:08.9796072Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:109:14
2019-09-12T09:20:08.9796131Z    |
2019-09-12T09:20:08.9796131Z    |
2019-09-12T09:20:08.9796176Z LL | const fn inc(x: &mut i32) { *x += 1 }
2019-09-12T09:20:08.9796282Z    |
2019-09-12T09:20:08.9796282Z    |
2019-09-12T09:20:08.9796601Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9796658Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9796690Z 
2019-09-12T09:20:08.9796740Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9797063Z    |
2019-09-12T09:20:08.9797063Z    |
2019-09-12T09:20:08.9797107Z LL | impl<T: std::fmt::Debug> Foo<T> {
2019-09-12T09:20:08.9797210Z    |
2019-09-12T09:20:08.9797210Z    |
2019-09-12T09:20:08.9797486Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9797558Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9797750Z 
2019-09-12T09:20:08.9797863Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9801011Z    |
2019-09-12T09:20:08.9801011Z    |
2019-09-12T09:20:08.9801057Z LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-09-12T09:20:08.9801160Z    |
2019-09-12T09:20:08.9801160Z    |
2019-09-12T09:20:08.9801553Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9801613Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9801666Z 
2019-09-12T09:20:08.9801715Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9802046Z    |
2019-09-12T09:20:08.9802046Z    |
2019-09-12T09:20:08.9802090Z LL | impl<T: Sync + Sized> Foo<T> {
2019-09-12T09:20:08.9802209Z    |
2019-09-12T09:20:08.9802209Z    |
2019-09-12T09:20:08.9802501Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9802558Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9802653Z error[E0723]: `impl Trait` in const fn is unstable
2019-09-12T09:20:08.9802910Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:130:24
2019-09-12T09:20:08.9802958Z    |
2019-09-12T09:20:08.9802958Z    |
2019-09-12T09:20:08.9803224Z LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-09-12T09:20:08.9803564Z    |
2019-09-12T09:20:08.9803564Z    |
2019-09-12T09:20:08.9803905Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9803962Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9803994Z 
2019-09-12T09:20:08.9804060Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9804392Z    |
2019-09-12T09:20:08.9804392Z    |
2019-09-12T09:20:08.9804452Z LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-09-12T09:20:08.9804545Z    |
2019-09-12T09:20:08.9804545Z    |
2019-09-12T09:20:08.9804842Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9804897Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9804929Z 
2019-09-12T09:20:08.9804994Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9805301Z    |
2019-09-12T09:20:08.9805301Z    |
2019-09-12T09:20:08.9805351Z LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-09-12T09:20:08.9805474Z    |
2019-09-12T09:20:08.9805474Z    |
2019-09-12T09:20:08.9805779Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9805836Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9805913Z error[E0723]: `impl Trait` in const fn is unstable
2019-09-12T09:20:08.9806502Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:135:23
2019-09-12T09:20:08.9806556Z    |
2019-09-12T09:20:08.9806556Z    |
2019-09-12T09:20:08.9806850Z LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
2019-09-12T09:20:08.9806964Z    |
2019-09-12T09:20:08.9806964Z    |
2019-09-12T09:20:08.9807250Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9807321Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9807353Z 
2019-09-12T09:20:08.9807401Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9809046Z    |
2019-09-12T09:20:08.9809046Z    |
2019-09-12T09:20:08.9809098Z LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-09-12T09:20:08.9809203Z    |
2019-09-12T09:20:08.9809203Z    |
2019-09-12T09:20:08.9809723Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9809790Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9809822Z 
2019-09-12T09:20:08.9809867Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9810182Z    |
2019-09-12T09:20:08.9810182Z    |
2019-09-12T09:20:08.9810411Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-09-12T09:20:08.9810531Z    |
2019-09-12T09:20:08.9810531Z    |
2019-09-12T09:20:08.9810807Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9810872Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9810947Z warning[E0515]: cannot return reference to temporary value
2019-09-12T09:20:08.9816747Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:137:63
2019-09-12T09:20:08.9827134Z    |
2019-09-12T09:20:08.9827134Z    |
2019-09-12T09:20:08.9827640Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-09-12T09:20:08.9827975Z    |                                                               ||
2019-09-12T09:20:08.9828032Z    |                                                               |temporary value created here
2019-09-12T09:20:08.9828255Z    |                                                               returns a reference to data owned by the current function
2019-09-12T09:20:08.9828339Z    |
2019-09-12T09:20:08.9828339Z    |
2019-09-12T09:20:08.9828404Z    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
2019-09-12T09:20:08.9828467Z    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
2019-09-12T09:20:08.9828737Z    = note: for more information, try `rustc --explain E0729`
2019-09-12T09:20:08.9828773Z 
2019-09-12T09:20:08.9828822Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-12T09:20:08.9829126Z    |
2019-09-12T09:20:08.9829126Z    |
2019-09-12T09:20:08.9829174Z LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-09-12T09:20:08.9829284Z    |
2019-09-12T09:20:08.9829284Z    |
2019-09-12T09:20:08.9829612Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9829692Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9829769Z error[E0723]: function pointers in const fn are unstable
2019-09-12T09:20:08.9830026Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:148:21
2019-09-12T09:20:08.9830083Z    |
2019-09-12T09:20:08.9830083Z    |
2019-09-12T09:20:08.9830125Z LL | const fn no_fn_ptrs(_x: fn()) {}
2019-09-12T09:20:08.9830224Z    |
2019-09-12T09:20:08.9830224Z    |
2019-09-12T09:20:08.9830680Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9830736Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9830829Z error[E0723]: function pointers in const fn are unstable
2019-09-12T09:20:08.9831082Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:150:27
2019-09-12T09:20:08.9831360Z    |
2019-09-12T09:20:08.9831360Z    |
2019-09-12T09:20:08.9831637Z LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-09-12T09:20:08.9831847Z    |
2019-09-12T09:20:08.9831847Z    |
2019-09-12T09:20:08.9832169Z    = note: for more information, see issue ***/issues/57563
2019-09-12T09:20:08.9832226Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-12T09:20:08.9832314Z error: aborting due to 36 previous errors
2019-09-12T09:20:08.9832560Z 
2019-09-12T09:20:08.9832604Z Some errors have detailed explanations: E0493, E0515, E0723.
2019-09-12T09:20:08.9832856Z For more information about an error, try `rustc --explain E0493`.
---
2019-09-12T09:20:08.9834593Z 17 
2019-09-12T09:20:08.9834620Z 
2019-09-12T09:20:08.9834646Z 
2019-09-12T09:20:08.9834691Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9835082Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/feature-gate-unleash_the_miri_inside_of_you.stderr
2019-09-12T09:20:08.9835336Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9835649Z To only update this specific test, also pass `--test-args consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs`
2019-09-12T09:20:08.9835738Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9835807Z status: exit code: 1
2019-09-12T09:20:08.9835807Z status: exit code: 1
2019-09-12T09:20:08.9836673Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9837008Z ------------------------------------------
2019-09-12T09:20:08.9837043Z 
2019-09-12T09:20:08.9837272Z ------------------------------------------
2019-09-12T09:20:08.9837318Z stderr:
2019-09-12T09:20:08.9837318Z stderr:
2019-09-12T09:20:08.9837537Z ------------------------------------------
2019-09-12T09:20:08.9837796Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9838082Z   --> /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:11:20
2019-09-12T09:20:08.9838136Z    |
2019-09-12T09:20:08.9838419Z LL |     const F: u32 = (U::X, 42).1; //~ ERROR destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9838477Z    |                    ^^^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9838510Z 
2019-09-12T09:20:08.9838556Z error: `std::vec::Vec::<T>::new` is not yet stable as a const fn
2019-09-12T09:20:08.9838843Z   --> /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:18:25
2019-09-12T09:20:08.9838894Z    |
2019-09-12T09:20:08.9838943Z LL |     const X: Vec<u32> = Vec::new(); //~ ERROR not yet stable as a const fn
2019-09-12T09:20:08.9839179Z    |
2019-09-12T09:20:08.9839295Z    = help: add `#![feature(const_vec_new)]` to the crate attributes to enable
2019-09-12T09:20:08.9839353Z 
2019-09-12T09:20:08.9839396Z error: aborting due to 2 previous errors
---
2019-09-12T09:20:08.9840745Z 9 
2019-09-12T09:20:08.9840781Z 
2019-09-12T09:20:08.9840808Z 
2019-09-12T09:20:08.9840864Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9841152Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/E0493.stderr
2019-09-12T09:20:08.9841410Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9841656Z To only update this specific test, also pass `--test-args span/E0493.rs`
2019-09-12T09:20:08.9841750Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9841796Z status: exit code: 1
2019-09-12T09:20:08.9841796Z status: exit code: 1
2019-09-12T09:20:08.9842492Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/E0493.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9842833Z ------------------------------------------
2019-09-12T09:20:08.9842882Z 
2019-09-12T09:20:08.9843095Z ------------------------------------------
2019-09-12T09:20:08.9843140Z stderr:
2019-09-12T09:20:08.9843140Z stderr:
2019-09-12T09:20:08.9843349Z ------------------------------------------
2019-09-12T09:20:08.9844026Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9844262Z   --> /checkout/src/test/ui/span/E0493.rs:17:17
2019-09-12T09:20:08.9844311Z    |
2019-09-12T09:20:08.9844371Z LL | const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
2019-09-12T09:20:08.9844425Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9845645Z error: aborting due to previous error
2019-09-12T09:20:08.9845699Z 
2019-09-12T09:20:08.9846042Z For more information about this error, try `rustc --explain E0493`.
2019-09-12T09:20:08.9846092Z 
---
2019-09-12T09:20:08.9847444Z 72 
2019-09-12T09:20:08.9847471Z 
2019-09-12T09:20:08.9847497Z 
2019-09-12T09:20:08.9847552Z The actual stderr differed from the expected stderr.
2019-09-12T09:20:08.9847850Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
2019-09-12T09:20:08.9848395Z To update references, rerun the tests and pass the `--bless` flag
2019-09-12T09:20:08.9848716Z To only update this specific test, also pass `--test-args static/static-drop-scope.rs`
2019-09-12T09:20:08.9848797Z error: 1 errors occurred comparing output.
2019-09-12T09:20:08.9848843Z status: exit code: 1
2019-09-12T09:20:08.9848843Z status: exit code: 1
2019-09-12T09:20:08.9849589Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary" "-A" "unused"
2019-09-12T09:20:08.9850066Z ------------------------------------------
2019-09-12T09:20:08.9850107Z 
2019-09-12T09:20:08.9850311Z ------------------------------------------
2019-09-12T09:20:08.9850367Z stderr:
2019-09-12T09:20:08.9850367Z stderr:
2019-09-12T09:20:08.9850572Z ------------------------------------------
2019-09-12T09:20:08.9850797Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9851035Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
2019-09-12T09:20:08.9851083Z    |
2019-09-12T09:20:08.9851530Z LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
2019-09-12T09:20:08.9851614Z    |                                                            ^^^^^^^^ statics cannot evaluate destructors
2019-09-12T09:20:08.9851698Z error[E0716]: temporary value dropped while borrowed
2019-09-12T09:20:08.9851948Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
2019-09-12T09:20:08.9851994Z    |
2019-09-12T09:20:08.9851994Z    |
2019-09-12T09:20:08.9852228Z LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
2019-09-12T09:20:08.9852556Z    |                                                      |     |       |
2019-09-12T09:20:08.9852611Z    |                                                      |     |       temporary value is freed at the end of this statement
2019-09-12T09:20:08.9852678Z    |                                                      |     creates a temporary which is freed while still in use
2019-09-12T09:20:08.9852678Z    |                                                      |     creates a temporary which is freed while still in use
2019-09-12T09:20:08.9852978Z    |                                                      using this value as a static requires that borrow lasts for `'static`
2019-09-12T09:20:08.9853237Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9854392Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
2019-09-12T09:20:08.9854442Z    |
2019-09-12T09:20:08.9854442Z    |
2019-09-12T09:20:08.9854690Z LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
2019-09-12T09:20:08.9855833Z    |                                                           ^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9855915Z error[E0716]: temporary value dropped while borrowed
2019-09-12T09:20:08.9856247Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
2019-09-12T09:20:08.9856295Z    |
2019-09-12T09:20:08.9856295Z    |
2019-09-12T09:20:08.9856536Z LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
2019-09-12T09:20:08.9856847Z    |                                                     |     |       |
2019-09-12T09:20:08.9856905Z    |                                                     |     |       temporary value is freed at the end of this statement
2019-09-12T09:20:08.9856980Z    |                                                     |     creates a temporary which is freed while still in use
2019-09-12T09:20:08.9857582Z    |                                                     using this value as a constant requires that borrow lasts for `'static`
2019-09-12T09:20:08.9857582Z    |                                                     using this value as a constant requires that borrow lasts for `'static`
2019-09-12T09:20:08.9857630Z 
2019-09-12T09:20:08.9857892Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9858149Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:17:28
2019-09-12T09:20:08.9858196Z    |
2019-09-12T09:20:08.9858243Z LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
2019-09-12T09:20:08.9858308Z    |                            ^^^^^^^^^^^^^ statics cannot evaluate destructors
2019-09-12T09:20:08.9858572Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9858820Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:20:27
2019-09-12T09:20:08.9858867Z    |
2019-09-12T09:20:08.9858867Z    |
2019-09-12T09:20:08.9858913Z LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;
2019-09-12T09:20:08.9858965Z    |                           ^^^^^^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9859265Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9859500Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:23:24
2019-09-12T09:20:08.9859557Z    |
2019-09-12T09:20:08.9859557Z    |
2019-09-12T09:20:08.9859600Z LL | const fn const_drop<T>(_: T) {}
2019-09-12T09:20:08.9859651Z    |                        ^ constant functions cannot evaluate destructors
2019-09-12T09:20:08.9859928Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9860160Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:27:5
2019-09-12T09:20:08.9860206Z    |
2019-09-12T09:20:08.9860206Z    |
2019-09-12T09:20:08.9860259Z LL |     (x, ()).1
2019-09-12T09:20:08.9860307Z    |     ^^^^^^^ constant functions cannot evaluate destructors
2019-09-12T09:20:08.9860565Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9860809Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:31:34
2019-09-12T09:20:08.9860864Z    |
2019-09-12T09:20:08.9860864Z    |
2019-09-12T09:20:08.9860919Z LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
2019-09-12T09:20:08.9860986Z    |                                  ^^^^^^^^^^^^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9861254Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-12T09:20:08.9861590Z   --> /checkout/src/test/ui/static/static-drop-scope.rs:36:43
2019-09-12T09:20:08.9861637Z    |
2019-09-12T09:20:08.9861637Z    |
2019-09-12T09:20:08.9861683Z LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
2019-09-12T09:20:08.9861737Z    |                                           ^^^^^^^^^^^ constants cannot evaluate destructors
2019-09-12T09:20:08.9861918Z error: aborting due to 10 previous errors
2019-09-12T09:20:08.9861948Z 
2019-09-12T09:20:08.9861993Z Some errors have detailed explanations: E0493, E0716.
2019-09-12T09:20:08.9862249Z For more information about an error, try `rustc --explain E0493`.
---
2019-09-12T09:20:08.9864611Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-12T09:20:08.9864783Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-12T09:20:08.9864897Z 
2019-09-12T09:20:08.9864930Z 
2019-09-12T09:20:08.9866451Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-12T09:20:08.9866710Z 
2019-09-12T09:20:08.9866743Z 
2019-09-12T09:20:08.9870252Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-12T09:20:08.9870355Z Build completed unsuccessfully in 1:11:39
2019-09-12T09:20:08.9870355Z Build completed unsuccessfully in 1:11:39
2019-09-12T09:20:08.9870456Z == clock drift check ==
2019-09-12T09:20:08.9870604Z   local time: Thu Sep 12 09:20:08 UTC 2019
2019-09-12T09:20:09.1320514Z   network time: Thu, 12 Sep 2019 09:20:09 GMT
2019-09-12T09:20:09.1321101Z == end clock drift check ==
2019-09-12T09:20:10.0060908Z ##[error]Bash exited with code '1'.
2019-09-12T09:20:10.0102988Z ##[section]Starting: Checkout
2019-09-12T09:20:10.0105554Z ==============================================================================
2019-09-12T09:20:10.0105662Z Task         : Get sources
2019-09-12T09:20:10.0105710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

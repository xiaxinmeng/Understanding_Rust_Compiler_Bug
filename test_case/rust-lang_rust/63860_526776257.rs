plain
2019-08-30T22:09:22.3654776Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T22:09:22.3845457Z ##[command]git config gc.auto 0
2019-08-30T22:09:22.3928997Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T22:09:22.3985916Z ##[command]git config --get-all http.proxy
2019-08-30T22:09:22.4125489Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-30T23:09:05.7269668Z .................................................................................................... 1200/8982
2019-08-30T23:09:12.1792763Z .................................................................................................... 1300/8982
2019-08-30T23:09:19.0570833Z ..................................................................F.FFF.............F............... 1400/8982
2019-08-30T23:09:25.5877205Z .................................................................................................... 1500/8982
2019-08-30T23:09:31.5566162Z .............FFFF.....................F.....................................................FF.F.... 1600/8982
2019-08-30T23:09:51.8587207Z .................................................................................................... 1800/8982
2019-08-30T23:09:51.8587207Z .................................................................................................... 1800/8982
2019-08-30T23:10:05.4638700Z .......................................iiiii........................................................ 1900/8982
2019-08-30T23:10:15.7164142Z .................................................................................................... 2100/8982
2019-08-30T23:10:18.1484272Z .................................................................................................... 2200/8982
2019-08-30T23:10:22.0675009Z .................................................................................................... 2300/8982
2019-08-30T23:10:29.2859487Z .................................................................................................... 2400/8982
---
2019-08-30T23:13:20.1016971Z ..........................i...............i......................................................... 4700/8982
2019-08-30T23:13:31.4665201Z .................................................................................................... 4800/8982
2019-08-30T23:13:37.3731827Z .................................................................................................... 4900/8982
2019-08-30T23:13:47.6851274Z .................................................................................................... 5000/8982
2019-08-30T23:13:53.0673923Z .......ii.ii........................................................................................ 5100/8982
2019-08-30T23:14:05.4462760Z .................................................................................................... 5300/8982
2019-08-30T23:14:13.3212861Z ......................................................................i............................. 5400/8982
2019-08-30T23:14:20.2727009Z .................................................................................................... 5500/8982
2019-08-30T23:14:26.8843809Z .................................................................................................... 5600/8982
2019-08-30T23:14:26.8843809Z .................................................................................................... 5600/8982
2019-08-30T23:14:36.7524773Z ................................................................ii...i..ii...........i.............. 5700/8982
2019-08-30T23:15:01.3229545Z .................................................................................................... 5900/8982
2019-08-30T23:15:09.7117313Z .................................................................................................... 6000/8982
2019-08-30T23:15:09.7117313Z .................................................................................................... 6000/8982
2019-08-30T23:15:15.5985274Z .................................................................i..ii.............................. 6100/8982
2019-08-30T23:15:43.2156080Z .................................................................................................... 6300/8982
2019-08-30T23:15:45.1402810Z ....................i............................................................................... 6400/8982
2019-08-30T23:15:47.1684643Z ............................................................................................i....... 6500/8982
2019-08-30T23:15:49.7792241Z .................................................................................................... 6600/8982
---
2019-08-30T23:19:36.6423415Z 
2019-08-30T23:19:36.6423956Z 1 error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6424438Z +   --> $DIR/const_let.rs:13:33
2019-08-30T23:19:36.6424747Z +    |
2019-08-30T23:19:36.6424940Z + LL | const X2: FakeNeedsDrop = { let x; x = FakeNeedsDrop; x };
2019-08-30T23:19:36.6425084Z +    |                                 ^ constants cannot evaluate destructors
2019-08-30T23:19:36.6425212Z + 
2019-08-30T23:19:36.6425740Z + error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6426644Z 3    |
2019-08-30T23:19:36.6426644Z 3    |
2019-08-30T23:19:36.6426798Z 4 LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2019-08-30T23:19:36.6426917Z 
2019-08-30T23:19:36.6427376Z 22 LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2019-08-30T23:19:36.6427582Z 23    |                      ^^^^^ constants cannot evaluate destructors
2019-08-30T23:19:36.6428243Z - error: aborting due to 4 previous errors
2019-08-30T23:19:36.6429168Z + error: aborting due to 5 previous errors
2019-08-30T23:19:36.6429552Z 26 
2019-08-30T23:19:36.6429728Z 27 
2019-08-30T23:19:36.6429728Z 27 
2019-08-30T23:19:36.6429902Z 
2019-08-30T23:19:36.6430044Z 
2019-08-30T23:19:36.6430359Z The actual stderr differed from the expected stderr.
2019-08-30T23:19:36.6431069Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/const_let.stderr
2019-08-30T23:19:36.6431630Z To update references, rerun the tests and pass the `--bless` flag
2019-08-30T23:19:36.6432627Z To only update this specific test, also pass `--test-args consts/const-eval/const_let.rs`
2019-08-30T23:19:36.6432989Z error: 1 errors occurred comparing output.
2019-08-30T23:19:36.6433252Z status: exit code: 1
2019-08-30T23:19:36.6433252Z status: exit code: 1
2019-08-30T23:19:36.6434553Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6435107Z ------------------------------------------
2019-08-30T23:19:36.6435249Z 
2019-08-30T23:19:36.6435880Z ------------------------------------------
2019-08-30T23:19:36.6436042Z stderr:
2019-08-30T23:19:36.6436042Z stderr:
2019-08-30T23:19:36.6436312Z ------------------------------------------
2019-08-30T23:19:36.6436647Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6436979Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:13:33
2019-08-30T23:19:36.6437128Z    |
2019-08-30T23:19:36.6437288Z LL | const X2: FakeNeedsDrop = { let x; x = FakeNeedsDrop; x };
2019-08-30T23:19:36.6437460Z    |                                 ^ constants cannot evaluate destructors
2019-08-30T23:19:36.6438064Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6439018Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:16:32
2019-08-30T23:19:36.6439282Z    |
2019-08-30T23:19:36.6439282Z    |
2019-08-30T23:19:36.6439452Z LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2019-08-30T23:19:36.6439624Z    |                                ^^^^^ constants cannot evaluate destructors
2019-08-30T23:19:36.6440225Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6440677Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:20:33
2019-08-30T23:19:36.6440886Z    |
2019-08-30T23:19:36.6440886Z    |
2019-08-30T23:19:36.6441079Z LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
2019-08-30T23:19:36.6441251Z    |                                 ^^^^^ constants cannot evaluate destructors
2019-08-30T23:19:36.6441824Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6442400Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:24:21
2019-08-30T23:19:36.6442940Z    |
2019-08-30T23:19:36.6442940Z    |
2019-08-30T23:19:36.6443237Z LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
2019-08-30T23:19:36.6443370Z    |                     ^^^^^ constants cannot evaluate destructors
2019-08-30T23:19:36.6443819Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-30T23:19:36.6444186Z   --> /checkout/src/test/ui/consts/const-eval/const_let.rs:28:22
2019-08-30T23:19:36.6444349Z    |
2019-08-30T23:19:36.6444349Z    |
2019-08-30T23:19:36.6444585Z LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
2019-08-30T23:19:36.6444757Z    |                      ^^^^^ constants cannot evaluate destructors
2019-08-30T23:19:36.6445041Z error: aborting due to 5 previous errors
2019-08-30T23:19:36.6445154Z 
2019-08-30T23:19:36.6445264Z 
2019-08-30T23:19:36.6445602Z ------------------------------------------
2019-08-30T23:19:36.6445602Z ------------------------------------------
2019-08-30T23:19:36.6445750Z 
2019-08-30T23:19:36.6445865Z 
2019-08-30T23:19:36.6446402Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-08-30T23:19:36.6446718Z 
2019-08-30T23:19:36.6446843Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6446989Z status: exit code: 101
2019-08-30T23:19:36.6447787Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6448295Z ------------------------------------------
2019-08-30T23:19:36.6448792Z 
2019-08-30T23:19:36.6449629Z ------------------------------------------
2019-08-30T23:19:36.6449869Z stderr:
2019-08-30T23:19:36.6449869Z stderr:
2019-08-30T23:19:36.6450280Z ------------------------------------------
2019-08-30T23:19:36.6450810Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6451338Z 
2019-08-30T23:19:36.6451586Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6451741Z 
2019-08-30T23:19:36.6451957Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6451957Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6452099Z 
2019-08-30T23:19:36.6453084Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6453771Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6456003Z 
2019-08-30T23:19:36.6456003Z 
2019-08-30T23:19:36.6456328Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6456397Z 
2019-08-30T23:19:36.6456568Z ------------------------------------------
2019-08-30T23:19:36.6456593Z 
2019-08-30T23:19:36.6456614Z 
2019-08-30T23:19:36.6456614Z 
2019-08-30T23:19:36.6456805Z ---- [ui] ui/consts/const-eval/const_panic.rs stdout ----
2019-08-30T23:19:36.6456831Z 
2019-08-30T23:19:36.6456869Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6456932Z status: exit code: 101
2019-08-30T23:19:36.6457528Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6457798Z ------------------------------------------
2019-08-30T23:19:36.6457825Z 
2019-08-30T23:19:36.6457993Z ------------------------------------------
2019-08-30T23:19:36.6458045Z stderr:
2019-08-30T23:19:36.6458045Z stderr:
2019-08-30T23:19:36.6458210Z ------------------------------------------
2019-08-30T23:19:36.6459534Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6459708Z 
2019-08-30T23:19:36.6459753Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6459809Z 
2019-08-30T23:19:36.6459855Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6459855Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6459885Z 
2019-08-30T23:19:36.6460295Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6460578Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6460610Z 
2019-08-30T23:19:36.6460610Z 
2019-08-30T23:19:36.6460904Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6460968Z 
2019-08-30T23:19:36.6461188Z ------------------------------------------
2019-08-30T23:19:36.6461221Z 
2019-08-30T23:19:36.6461262Z 
2019-08-30T23:19:36.6461262Z 
2019-08-30T23:19:36.6461500Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-08-30T23:19:36.6461535Z 
2019-08-30T23:19:36.6461582Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6461656Z status: exit code: 101
2019-08-30T23:19:36.6472938Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6473366Z ------------------------------------------
2019-08-30T23:19:36.6473399Z 
2019-08-30T23:19:36.6473592Z ------------------------------------------
2019-08-30T23:19:36.6473628Z stderr:
2019-08-30T23:19:36.6473628Z stderr:
2019-08-30T23:19:36.6473785Z ------------------------------------------
2019-08-30T23:19:36.6476051Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6476152Z 
2019-08-30T23:19:36.6476245Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6476271Z 
2019-08-30T23:19:36.6476306Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6476306Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6476328Z 
2019-08-30T23:19:36.6476631Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6476861Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6477074Z 
2019-08-30T23:19:36.6477074Z 
2019-08-30T23:19:36.6477292Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6477342Z 
2019-08-30T23:19:36.6477758Z ------------------------------------------
2019-08-30T23:19:36.6477784Z 
2019-08-30T23:19:36.6477804Z 
2019-08-30T23:19:36.6477804Z 
2019-08-30T23:19:36.6477991Z ---- [ui] ui/consts/const-eval/feature-gate-const_panic.rs stdout ----
2019-08-30T23:19:36.6478018Z 
2019-08-30T23:19:36.6478071Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6478111Z status: exit code: 101
2019-08-30T23:19:36.6479503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/feature-gate-const_panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/feature-gate-const_panic/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6479919Z ------------------------------------------
2019-08-30T23:19:36.6479974Z 
2019-08-30T23:19:36.6480190Z ------------------------------------------
2019-08-30T23:19:36.6480237Z stderr:
2019-08-30T23:19:36.6480237Z stderr:
2019-08-30T23:19:36.6480461Z ------------------------------------------
2019-08-30T23:19:36.6480512Z error[E0658]: panicking in constants is unstable
2019-08-30T23:19:36.6480766Z   --> /checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs:3:15
2019-08-30T23:19:36.6480834Z    |
2019-08-30T23:19:36.6480882Z LL | const Z: () = panic!("cheese");
2019-08-30T23:19:36.6480999Z    |
2019-08-30T23:19:36.6480999Z    |
2019-08-30T23:19:36.6481294Z    = note: for more information, see ***/issues/51999
2019-08-30T23:19:36.6481351Z    = help: add `#![feature(const_panic)]` to the crate attributes to enable
2019-08-30T23:19:36.6481749Z 
2019-08-30T23:19:36.6481749Z 
2019-08-30T23:19:36.6482243Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6482523Z error: aborting due to previous error
2019-08-30T23:19:36.6482545Z 
2019-08-30T23:19:36.6483006Z For more information about this error, try `rustc --explain E0658`.
2019-08-30T23:19:36.6483035Z 
2019-08-30T23:19:36.6483035Z 
2019-08-30T23:19:36.6483078Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6483101Z 
2019-08-30T23:19:36.6483138Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6483177Z 
2019-08-30T23:19:36.6483421Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6483773Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6483801Z 
2019-08-30T23:19:36.6483801Z 
2019-08-30T23:19:36.6484017Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6484082Z 
2019-08-30T23:19:36.6484249Z ------------------------------------------
2019-08-30T23:19:36.6484275Z 
2019-08-30T23:19:36.6484295Z 
2019-08-30T23:19:36.6484295Z 
2019-08-30T23:19:36.6484480Z ---- [ui] ui/consts/const_arg_local.rs stdout ----
2019-08-30T23:19:36.6484506Z 
2019-08-30T23:19:36.6484544Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6484588Z status: exit code: 101
2019-08-30T23:19:36.6485174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_local.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6485620Z ------------------------------------------
2019-08-30T23:19:36.6485647Z 
2019-08-30T23:19:36.6485810Z ------------------------------------------
2019-08-30T23:19:36.6485862Z stderr:
2019-08-30T23:19:36.6485862Z stderr:
2019-08-30T23:19:36.6486022Z ------------------------------------------
2019-08-30T23:19:36.6486321Z thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 3', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-30T23:19:36.6486423Z 
2019-08-30T23:19:36.6486457Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6486479Z 
2019-08-30T23:19:36.6486530Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6486530Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6486559Z 
2019-08-30T23:19:36.6486825Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6487055Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6487081Z 
2019-08-30T23:19:36.6487081Z 
2019-08-30T23:19:36.6487471Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6487535Z 
2019-08-30T23:19:36.6487700Z ------------------------------------------
2019-08-30T23:19:36.6487725Z 
2019-08-30T23:19:36.6487745Z 
2019-08-30T23:19:36.6487745Z 
2019-08-30T23:19:36.6487944Z ---- [ui] ui/consts/const_arg_promotable2.rs stdout ----
2019-08-30T23:19:36.6487972Z 
2019-08-30T23:19:36.6488009Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6488061Z status: exit code: 101
2019-08-30T23:19:36.6499075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_promotable2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable2/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6499612Z ------------------------------------------
2019-08-30T23:19:36.6499652Z 
2019-08-30T23:19:36.6502737Z ------------------------------------------
2019-08-30T23:19:36.6502787Z stderr:
2019-08-30T23:19:36.6502787Z stderr:
2019-08-30T23:19:36.6502960Z ------------------------------------------
2019-08-30T23:19:36.6503201Z error: internal compiler error: src/librustc_mir/interpret/eval_context.rs:139: The type checker should prevent reading from a never-written local
2019-08-30T23:19:36.6503612Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-30T23:19:36.6503658Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T23:19:36.6503713Z error: aborting due to previous error
2019-08-30T23:19:36.6503736Z 
2019-08-30T23:19:36.6503736Z 
2019-08-30T23:19:36.6503755Z 
2019-08-30T23:19:36.6503790Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6503829Z 
2019-08-30T23:19:36.6504088Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6504328Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6504354Z 
2019-08-30T23:19:36.6504354Z 
2019-08-30T23:19:36.6504564Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6504612Z 
2019-08-30T23:19:36.6504786Z ------------------------------------------
2019-08-30T23:19:36.6504858Z 
2019-08-30T23:19:36.6504879Z 
2019-08-30T23:19:36.6504879Z 
2019-08-30T23:19:36.6505056Z ---- [ui] ui/consts/const_arg_promotable.rs stdout ----
2019-08-30T23:19:36.6505097Z 
2019-08-30T23:19:36.6505130Z error: ui test compiled successfully!
2019-08-30T23:19:36.6505164Z status: exit code: 0
2019-08-30T23:19:36.6505811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_promotable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6506090Z ------------------------------------------
2019-08-30T23:19:36.6506125Z 
2019-08-30T23:19:36.6506288Z ------------------------------------------
2019-08-30T23:19:36.6506338Z stderr:
2019-08-30T23:19:36.6506338Z stderr:
2019-08-30T23:19:36.6506497Z ------------------------------------------
2019-08-30T23:19:36.6506521Z 
2019-08-30T23:19:36.6506675Z ------------------------------------------
2019-08-30T23:19:36.6506699Z 
2019-08-30T23:19:36.6506734Z 
2019-08-30T23:19:36.6506897Z ---- [ui] ui/consts/const_arg_wrapper.rs stdout ----
2019-08-30T23:19:36.6507096Z 
2019-08-30T23:19:36.6507133Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6507187Z status: exit code: 101
2019-08-30T23:19:36.6507974Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_wrapper.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6508244Z ------------------------------------------
2019-08-30T23:19:36.6508272Z 
2019-08-30T23:19:36.6508635Z ------------------------------------------
2019-08-30T23:19:36.6508850Z stderr:
2019-08-30T23:19:36.6508850Z stderr:
2019-08-30T23:19:36.6509285Z ------------------------------------------
2019-08-30T23:19:36.6509605Z thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 3', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-30T23:19:36.6509721Z 
2019-08-30T23:19:36.6509783Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6509813Z 
2019-08-30T23:19:36.6509858Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6509858Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6509999Z 
2019-08-30T23:19:36.6510369Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6510648Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6510698Z 
2019-08-30T23:19:36.6510698Z 
2019-08-30T23:19:36.6510974Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6511037Z 
2019-08-30T23:19:36.6511260Z ------------------------------------------
2019-08-30T23:19:36.6511292Z 
2019-08-30T23:19:36.6511317Z 
2019-08-30T23:19:36.6511317Z 
2019-08-30T23:19:36.6511536Z ---- [ui] ui/consts/invalid_promotion.rs stdout ----
2019-08-30T23:19:36.6511579Z 
2019-08-30T23:19:36.6511819Z error: test compilation failed although it shouldn't!
2019-08-30T23:19:36.6511869Z status: exit code: 101
2019-08-30T23:19:36.6512832Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid_promotion/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6513087Z ------------------------------------------
2019-08-30T23:19:36.6513131Z 
2019-08-30T23:19:36.6513293Z ------------------------------------------
2019-08-30T23:19:36.6513396Z stderr:
2019-08-30T23:19:36.6513396Z stderr:
2019-08-30T23:19:36.6513601Z ------------------------------------------
2019-08-30T23:19:36.6513841Z error: internal compiler error: src/librustc/mir/tcx.rs:84: deref projection of non-dereferencable ty PlaceTy { ty: Hz, variant_index: None }
2019-08-30T23:19:36.6514072Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
2019-08-30T23:19:36.6514125Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T23:19:36.6514162Z error: aborting due to previous error
2019-08-30T23:19:36.6514184Z 
2019-08-30T23:19:36.6514184Z 
2019-08-30T23:19:36.6514219Z 
2019-08-30T23:19:36.6514253Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6514275Z 
2019-08-30T23:19:36.6514528Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6514736Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6514761Z 
2019-08-30T23:19:36.6514761Z 
2019-08-30T23:19:36.6515003Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
2019-08-30T23:19:36.6515054Z 
2019-08-30T23:19:36.6515214Z ------------------------------------------
2019-08-30T23:19:36.6515238Z 
2019-08-30T23:19:36.6515271Z 
2019-08-30T23:19:36.6515271Z 
2019-08-30T23:19:36.6515466Z ---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail.rs stdout ----
2019-08-30T23:19:36.6515494Z 
2019-08-30T23:19:36.6515526Z error: ui test compiled successfully!
2019-08-30T23:19:36.6515574Z status: exit code: 0
2019-08-30T23:19:36.6516175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/nll-fail/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6516425Z ------------------------------------------
2019-08-30T23:19:36.6516522Z 
2019-08-30T23:19:36.6516723Z ------------------------------------------
2019-08-30T23:19:36.6516759Z stderr:
---
2019-08-30T23:19:36.6517703Z ---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail.rs stdout ----
2019-08-30T23:19:36.6517748Z 
2019-08-30T23:19:36.6517782Z error: ui test compiled successfully!
2019-08-30T23:19:36.6517816Z status: exit code: 0
2019-08-30T23:19:36.6518675Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=migrate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/migrate-fail/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6519377Z ------------------------------------------
2019-08-30T23:19:36.6519413Z 
2019-08-30T23:19:36.6519626Z ------------------------------------------
2019-08-30T23:19:36.6519696Z stderr:
---
2019-08-30T23:19:36.6520606Z ---- [ui] ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs stdout ----
2019-08-30T23:19:36.6520644Z 
2019-08-30T23:19:36.6520706Z error: ui test compiled successfully!
2019-08-30T23:19:36.6520761Z status: exit code: 0
2019-08-30T23:19:36.6521579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rfc-2203-const-array-repeat-exprs/trait-error/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6521921Z ------------------------------------------
2019-08-30T23:19:36.6521955Z 
2019-08-30T23:19:36.6522165Z ------------------------------------------
2019-08-30T23:19:36.6522211Z stderr:
2019-08-30T23:19:36.6522211Z stderr:
2019-08-30T23:19:36.6522583Z ------------------------------------------
2019-08-30T23:19:36.6522615Z 
2019-08-30T23:19:36.6522777Z ------------------------------------------
2019-08-30T23:19:36.6522802Z 
2019-08-30T23:19:36.6522836Z 
2019-08-30T23:19:36.6523017Z ---- [ui] ui/consts/uninhabited-const-issue-61744.rs stdout ----
2019-08-30T23:19:36.6523046Z 
2019-08-30T23:19:36.6523084Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6523135Z status: exit code: 101
2019-08-30T23:19:36.6523726Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/uninhabited-const-issue-61744.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/uninhabited-const-issue-61744/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6524073Z ------------------------------------------
2019-08-30T23:19:36.6524101Z 
2019-08-30T23:19:36.6524282Z ------------------------------------------
2019-08-30T23:19:36.6524318Z stderr:
2019-08-30T23:19:36.6524318Z stderr:
2019-08-30T23:19:36.6524478Z ------------------------------------------
2019-08-30T23:19:36.6524751Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6524837Z 
2019-08-30T23:19:36.6524888Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6524911Z 
2019-08-30T23:19:36.6524946Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6524946Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6524969Z 
2019-08-30T23:19:36.6525235Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6525463Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6525489Z 
2019-08-30T23:19:36.6525489Z 
2019-08-30T23:19:36.6525716Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6525765Z 
2019-08-30T23:19:36.6525926Z ------------------------------------------
2019-08-30T23:19:36.6525965Z 
2019-08-30T23:19:36.6525985Z 
2019-08-30T23:19:36.6525985Z 
2019-08-30T23:19:36.6526166Z ---- [ui] ui/functions-closures/fn-item-type-zero-sized.rs stdout ----
2019-08-30T23:19:36.6526193Z 
2019-08-30T23:19:36.6526453Z error: test compilation failed although it shouldn't!
2019-08-30T23:19:36.6526500Z status: exit code: 101
2019-08-30T23:19:36.6527292Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functions-closures/fn-item-type-zero-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-item-type-zero-sized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-item-type-zero-sized/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6527566Z ------------------------------------------
2019-08-30T23:19:36.6527611Z 
2019-08-30T23:19:36.6527780Z ------------------------------------------
2019-08-30T23:19:36.6527817Z stderr:
2019-08-30T23:19:36.6527817Z stderr:
2019-08-30T23:19:36.6528003Z ------------------------------------------
2019-08-30T23:19:36.6528247Z thread 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 50', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-30T23:19:36.6528515Z 
2019-08-30T23:19:36.6528559Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6528583Z 
2019-08-30T23:19:36.6528766Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6528766Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6528977Z 
2019-08-30T23:19:36.6529353Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6529649Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6529682Z 
2019-08-30T23:19:36.6529682Z 
2019-08-30T23:19:36.6529957Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6530020Z 
2019-08-30T23:19:36.6530256Z ------------------------------------------
2019-08-30T23:19:36.6530290Z 
2019-08-30T23:19:36.6530315Z 
2019-08-30T23:19:36.6530315Z 
2019-08-30T23:19:36.6530534Z ---- [ui] ui/issues/issue-32829.rs stdout ----
2019-08-30T23:19:36.6530585Z 
2019-08-30T23:19:36.6530632Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6530783Z status: exit code: 101
2019-08-30T23:19:36.6531545Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32829.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6531877Z ------------------------------------------
2019-08-30T23:19:36.6531913Z 
2019-08-30T23:19:36.6532310Z ------------------------------------------
2019-08-30T23:19:36.6532351Z stderr:
2019-08-30T23:19:36.6532351Z stderr:
2019-08-30T23:19:36.6532701Z ------------------------------------------
2019-08-30T23:19:36.6532741Z error[E0658]: panicking in statics is unstable
2019-08-30T23:19:36.6532932Z   --> /checkout/src/test/ui/issues/issue-32829.rs:1:22
2019-08-30T23:19:36.6532986Z    |
2019-08-30T23:19:36.6533023Z LL | static S : u64 = { { panic!("foo"); 0 } };
2019-08-30T23:19:36.6533110Z    |
2019-08-30T23:19:36.6533110Z    |
2019-08-30T23:19:36.6533332Z    = note: for more information, see ***/issues/51999
2019-08-30T23:19:36.6533378Z    = help: add `#![feature(const_panic)]` to the crate attributes to enable
2019-08-30T23:19:36.6533766Z 
2019-08-30T23:19:36.6533766Z 
2019-08-30T23:19:36.6534093Z thread 'rustc' panicked at '`apply_call_return_effect` must not be called unless cursor is pointing at a `Call` terminator', src/librustc_mir/dataflow/generic.rs:227:9
2019-08-30T23:19:36.6534219Z error: aborting due to previous error
2019-08-30T23:19:36.6534244Z 
2019-08-30T23:19:36.6534474Z For more information about this error, try `rustc --explain E0658`.
2019-08-30T23:19:36.6534504Z 
2019-08-30T23:19:36.6534504Z 
2019-08-30T23:19:36.6534541Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6534566Z 
2019-08-30T23:19:36.6534621Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6534646Z 
2019-08-30T23:19:36.6534909Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6535162Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6535191Z 
2019-08-30T23:19:36.6535191Z 
2019-08-30T23:19:36.6535431Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6535501Z 
2019-08-30T23:19:36.6535689Z ------------------------------------------
2019-08-30T23:19:36.6535716Z 
2019-08-30T23:19:36.6535737Z 
2019-08-30T23:19:36.6535737Z 
2019-08-30T23:19:36.6535953Z ---- [ui] ui/rustc-args-required-const.rs stdout ----
2019-08-30T23:19:36.6535981Z 
2019-08-30T23:19:36.6536021Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6536060Z status: exit code: 101
2019-08-30T23:19:36.6536674Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rustc-args-required-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-args-required-const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rustc-args-required-const/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6536956Z ------------------------------------------
2019-08-30T23:19:36.6536985Z 
2019-08-30T23:19:36.6537271Z ------------------------------------------
2019-08-30T23:19:36.6537327Z stderr:
2019-08-30T23:19:36.6537327Z stderr:
2019-08-30T23:19:36.6537508Z ------------------------------------------
2019-08-30T23:19:36.6537757Z thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 5', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-30T23:19:36.6537854Z 
2019-08-30T23:19:36.6537891Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6537915Z 
2019-08-30T23:19:36.6537968Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6537968Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6537994Z 
2019-08-30T23:19:36.6538263Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6539082Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6539121Z 
2019-08-30T23:19:36.6539121Z 
2019-08-30T23:19:36.6539410Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6539502Z 
2019-08-30T23:19:36.6539716Z ------------------------------------------
2019-08-30T23:19:36.6539749Z 
2019-08-30T23:19:36.6539789Z 
2019-08-30T23:19:36.6539789Z 
2019-08-30T23:19:36.6540016Z ---- [ui] ui/unsized-locals/borrow-after-move.rs stdout ----
2019-08-30T23:19:36.6540049Z 
2019-08-30T23:19:36.6540098Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-30T23:19:36.6540163Z status: exit code: 101
2019-08-30T23:19:36.6541004Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/borrow-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move/auxiliary" "-A" "unused"
2019-08-30T23:19:36.6541378Z ------------------------------------------
2019-08-30T23:19:36.6541413Z 
2019-08-30T23:19:36.6541648Z ------------------------------------------
2019-08-30T23:19:36.6541695Z stderr:
2019-08-30T23:19:36.6541695Z stderr:
2019-08-30T23:19:36.6541905Z ------------------------------------------
2019-08-30T23:19:36.6542826Z thread 'rustc' panicked at 'index out of bounds: the len is 2 but the index is 6', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-30T23:19:36.6542922Z 
2019-08-30T23:19:36.6542957Z error: internal compiler error: unexpected panic
2019-08-30T23:19:36.6542996Z 
2019-08-30T23:19:36.6543031Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6543031Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T23:19:36.6543056Z 
2019-08-30T23:19:36.6543363Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T23:19:36.6543596Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T23:19:36.6543623Z 
2019-08-30T23:19:36.6543623Z 
2019-08-30T23:19:36.6543854Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-30T23:19:36.6543904Z 
2019-08-30T23:19:36.6544069Z ------------------------------------------
2019-08-30T23:19:36.6544110Z 
2019-08-30T23:19:36.6544131Z 
---
2019-08-30T23:19:36.6547976Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-30T23:19:36.6548024Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T23:19:36.6548050Z 
2019-08-30T23:19:36.6548087Z 
2019-08-30T23:19:36.6553130Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-30T23:19:36.6553627Z 
2019-08-30T23:19:36.6553651Z 
2019-08-30T23:19:36.6553686Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-30T23:19:36.6553740Z Build completed unsuccessfully in 1:03:13
2019-08-30T23:19:36.6553740Z Build completed unsuccessfully in 1:03:13
2019-08-30T23:19:36.6553813Z == clock drift check ==
2019-08-30T23:19:36.6576878Z   local time: Fri Aug 30 23:19:36 UTC 2019
2019-08-30T23:19:36.9340625Z   network time: Fri, 30 Aug 2019 23:19:36 GMT
2019-08-30T23:19:36.9340858Z == end clock drift check ==
2019-08-30T23:19:37.6902438Z ##[error]Bash exited with code '1'.
2019-08-30T23:19:37.6935467Z ##[section]Starting: Checkout
2019-08-30T23:19:37.6936938Z ==============================================================================
2019-08-30T23:19:37.6936981Z Task         : Get sources
2019-08-30T23:19:37.6937019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

plain
2019-09-25T20:25:58.5460874Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T20:25:58.5634646Z ##[command]git config gc.auto 0
2019-09-25T20:25:58.5708466Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T20:25:58.5765973Z ##[command]git config --get-all http.proxy
2019-09-25T20:25:58.5912102Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-25T21:25:22.5876860Z ........................F........................................................................... 1500/9046
2019-09-25T21:25:28.5676293Z ...................................................................................................F 1600/9046
2019-09-25T21:25:40.3202899Z ....................................F.......................................i...............i....... 1700/9046
2019-09-25T21:25:47.0790967Z .................................................................................................... 1800/9046
2019-09-25T21:25:55.3797624Z ...................................................................iiiii............................ 1900/9046
2019-09-25T21:26:14.4121793Z .................................................................................................... 2100/9046
2019-09-25T21:26:16.7493765Z .................................................................................................... 2200/9046
2019-09-25T21:26:19.5748031Z .................................................................................................... 2300/9046
2019-09-25T21:26:27.2132543Z .................................................................................................... 2400/9046
---
2019-09-25T21:29:18.3156229Z ..........................................................i...............i......................... 4700/9046
2019-09-25T21:29:27.3862455Z .................................................................................................... 4800/9046
2019-09-25T21:29:36.0419511Z .................................................................................................... 4900/9046
2019-09-25T21:29:43.0959208Z .................................................................................................... 5000/9046
2019-09-25T21:29:53.0414205Z .............................................ii.ii.................................................. 5100/9046
2019-09-25T21:30:02.6080607Z .................................................................................................... 5300/9046
2019-09-25T21:30:12.1181073Z .................................................................................................... 5400/9046
2019-09-25T21:30:19.0449318Z ..........i......................................................................................... 5500/9046
2019-09-25T21:30:24.1523153Z .................................................................................................... 5600/9046
2019-09-25T21:30:24.1523153Z .................................................................................................... 5600/9046
2019-09-25T21:30:35.4925373Z .................................................................................................... 5700/9046
2019-09-25T21:30:47.8514094Z .....ii...i..ii...........i......................................................................... 5800/9046
2019-09-25T21:31:09.2536962Z .................................................................................................... 6000/9046
2019-09-25T21:31:17.6704114Z .................................................................................................... 6100/9046
2019-09-25T21:31:17.6704114Z .................................................................................................... 6100/9046
2019-09-25T21:31:30.1709002Z .......i..ii........................................................................................ 6200/9046
2019-09-25T21:31:47.5978210Z ...................................................................i................................ 6400/9046
2019-09-25T21:31:49.5749506Z .................................................................................................... 6500/9046
2019-09-25T21:31:51.9655755Z .......................................i............................................................ 6600/9046
2019-09-25T21:31:55.9492522Z .................................................................................................... 6700/9046
---
2019-09-25T21:32:48.7981562Z .................................................................................................... 7200/9046
2019-09-25T21:32:53.7004269Z .................................................................................................... 7300/9046
2019-09-25T21:33:00.4959499Z .................................................................................................... 7400/9046
2019-09-25T21:33:10.1093673Z .................................................................................................... 7500/9046
2019-09-25T21:33:19.6925986Z ...........................................................................................ii......i 7600/9046
2019-09-25T21:33:30.7324099Z .................................................................................................... 7800/9046
2019-09-25T21:33:47.4206012Z .................................................................................................... 7900/9046
2019-09-25T21:33:55.8679960Z .................................................................................................... 8000/9046
2019-09-25T21:34:05.3648449Z .................................................................................................... 8100/9046
---
2019-09-25T21:35:47.6959332Z failures:
2019-09-25T21:35:47.6997174Z 
2019-09-25T21:35:47.6997975Z ---- [ui] ui/consts/const-eval/assign-to-static-within-other-static-2.rs stdout ----
2019-09-25T21:35:47.6998163Z 
2019-09-25T21:35:47.6998338Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-09-25T21:35:47.6998463Z status: exit code: 101
2019-09-25T21:35:47.6999459Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/assign-to-static-within-other-static-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/assign-to-static-within-other-static-2/auxiliary" "-A" "unused"
2019-09-25T21:35:47.7000860Z ------------------------------------------
2019-09-25T21:35:47.7001130Z 
2019-09-25T21:35:47.7001539Z ------------------------------------------
2019-09-25T21:35:47.7001765Z stderr:
2019-09-25T21:35:47.7001765Z stderr:
2019-09-25T21:35:47.7002138Z ------------------------------------------
2019-09-25T21:35:47.7002344Z error[E0019]: static contains unimplemented expression type
2019-09-25T21:35:47.7002817Z   --> /checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs:16:5
2019-09-25T21:35:47.7003051Z    |
2019-09-25T21:35:47.7003240Z LL |     *FOO.0.get() = 5; //~ ERROR contains unimplemented expression type
2019-09-25T21:35:47.7003849Z 
2019-09-25T21:35:47.7003849Z 
2019-09-25T21:35:47.7004254Z [ERROR rustc_mir::transform::qualify_consts] old validator: [(/checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs:16:5: 16:21, "MutDeref")]
2019-09-25T21:35:47.7004924Z [ERROR rustc_mir::transform::qualify_consts] new validator: [(/checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs:16:6: 16:11, "MutBorrow(Shared)"), (/checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs:16:5: 16:21, "MutDeref")]
2019-09-25T21:35:47.7005548Z error: internal compiler error: src/librustc_mir/transform/qualify_consts.rs:1038: Disagreement between legacy and dataflow-based const validators.
2019-09-25T21:35:47.7005947Z     After filing an issue, use `-Zsuppress-const-validation-back-compat-ice` to compile your code.
2019-09-25T21:35:47.7006381Z   --> /checkout/src/test/ui/consts/const-eval/assign-to-static-within-other-static-2.rs:15:1
2019-09-25T21:35:47.7006580Z    |
2019-09-25T21:35:47.7006896Z LL | / static BAR: () = unsafe {
2019-09-25T21:35:47.7007032Z LL | |     *FOO.0.get() = 5; //~ ERROR contains unimplemented expression type
2019-09-25T21:35:47.7007299Z    | |__^
2019-09-25T21:35:47.7007407Z 
2019-09-25T21:35:47.7007757Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
2019-09-25T21:35:47.7007953Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7007953Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7008091Z 
2019-09-25T21:35:47.7008216Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-25T21:35:47.7008325Z 
2019-09-25T21:35:47.7009019Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-25T21:35:47.7009527Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-25T21:35:47.7010221Z 
2019-09-25T21:35:47.7010221Z 
2019-09-25T21:35:47.7011247Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-25T21:35:47.7011662Z error: aborting due to 2 previous errors
2019-09-25T21:35:47.7011794Z 
2019-09-25T21:35:47.7012210Z For more information about this error, try `rustc --explain E0019`.
2019-09-25T21:35:47.7012474Z 
2019-09-25T21:35:47.7012474Z 
2019-09-25T21:35:47.7012898Z ------------------------------------------
2019-09-25T21:35:47.7013077Z 
2019-09-25T21:35:47.7013216Z 
2019-09-25T21:35:47.7013906Z ---- [ui] ui/consts/const-eval/mod-static-with-const-fn.rs stdout ----
2019-09-25T21:35:47.7014048Z 
2019-09-25T21:35:47.7014193Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-09-25T21:35:47.7014310Z status: exit code: 101
2019-09-25T21:35:47.7015165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/mod-static-with-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/mod-static-with-const-fn/auxiliary" "-A" "unused"
2019-09-25T21:35:47.7015787Z ------------------------------------------
2019-09-25T21:35:47.7015935Z 
2019-09-25T21:35:47.7016222Z ------------------------------------------
2019-09-25T21:35:47.7016376Z stderr:
2019-09-25T21:35:47.7016376Z stderr:
2019-09-25T21:35:47.7016678Z ------------------------------------------
2019-09-25T21:35:47.7016860Z error[E0019]: static contains unimplemented expression type
2019-09-25T21:35:47.7017189Z   --> /checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:18:5
2019-09-25T21:35:47.7017361Z    |
2019-09-25T21:35:47.7017483Z LL |     *FOO.0.get() = 5;
2019-09-25T21:35:47.7017716Z 
2019-09-25T21:35:47.7017845Z error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-25T21:35:47.7018319Z   --> /checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:21:5
2019-09-25T21:35:47.7018491Z    |
2019-09-25T21:35:47.7018491Z    |
2019-09-25T21:35:47.7018640Z LL |     foo();
2019-09-25T21:35:47.7018751Z    |     ^^^^^
2019-09-25T21:35:47.7018846Z 
2019-09-25T21:35:47.7019360Z [ERROR rustc_mir::transform::qualify_consts] old validator: [(/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:18:5: 18:21, "MutDeref"), (/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:21:5: 21:10, "FnCallNonConst(DefId(0:19 ~ mod_static_with_const_fn[317d]::foo[0]))")]
2019-09-25T21:35:47.7020404Z [ERROR rustc_mir::transform::qualify_consts] new validator: [(/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:18:6: 18:11, "MutBorrow(Shared)"), (/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:18:5: 18:21, "MutDeref"), (/checkout/src/test/ui/consts/const-eval/mod-static-with-const-fn.rs:21:5: 21:10, "FnCallNonConst(DefId(0:19 ~ mod_static_with_const_fn[317d]::foo[0]))")]
2019-09-25T21:35:47.7021084Z error: internal compiler error: src/librustc_mir/transform/qualify_consts.rs:1038: Disagreement between legacy and dataflow-based const validators.
2019-09-25T21:35:47.7021604Z     After filing an issue, use `-Zsuppress-const-validation-back-compat-ice` to compile your code.
2019-09-25T21:35:47.7022327Z    |
2019-09-25T21:35:47.7022327Z    |
2019-09-25T21:35:47.7022491Z LL | / static BAR: () = unsafe {
2019-09-25T21:35:47.7022662Z LL | |     *FOO.0.get() = 5;
2019-09-25T21:35:47.7022834Z LL | |     //~^ contains unimplemented expression
2019-09-25T21:35:47.7023372Z LL | |     foo();
2019-09-25T21:35:47.7023372Z LL | |     foo();
2019-09-25T21:35:47.7023724Z LL | |     //~^ ERROR calls in statics are limited to constant functions, tuple structs and tuple variants
2019-09-25T21:35:47.7023984Z    | |__^
2019-09-25T21:35:47.7024088Z 
2019-09-25T21:35:47.7024447Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
2019-09-25T21:35:47.7024646Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7024646Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7024778Z 
2019-09-25T21:35:47.7025075Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-25T21:35:47.7025178Z 
2019-09-25T21:35:47.7025561Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-25T21:35:47.7026094Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-25T21:35:47.7026336Z 
2019-09-25T21:35:47.7026336Z 
2019-09-25T21:35:47.7026712Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-25T21:35:47.7026995Z error: aborting due to 3 previous errors
2019-09-25T21:35:47.7027094Z 
2019-09-25T21:35:47.7027360Z Some errors have detailed explanations: E0015, E0019.
2019-09-25T21:35:47.7027752Z For more information about an error, try `rustc --explain E0015`.
---
2019-09-25T21:35:47.7029287Z 6 
2019-09-25T21:35:47.7029405Z 7 warning: skipping const checks
2019-09-25T21:35:47.7029701Z -   --> $DIR/mutable_references.rs:8:25
2019-09-25T21:35:47.7029995Z -    |
2019-09-25T21:35:47.7030306Z - LL | static FOO: &&mut u32 = &&mut 42;
2019-09-25T21:35:47.7030969Z - 
2019-09-25T21:35:47.7031299Z - warning: skipping const checks
2019-09-25T21:35:47.7031620Z 14   --> $DIR/mutable_references.rs:12:23
2019-09-25T21:35:47.7031779Z 15    |
2019-09-25T21:35:47.7031779Z 15    |
2019-09-25T21:35:47.7031902Z 16 LL | static BAR: &mut () = &mut ();
2019-09-25T21:35:47.7032120Z 
2019-09-25T21:35:47.7032236Z The actual stderr differed from the expected stderr.
2019-09-25T21:35:47.7032617Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
2019-09-25T21:35:47.7032617Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
2019-09-25T21:35:47.7032978Z To update references, rerun the tests and pass the `--bless` flag
2019-09-25T21:35:47.7033402Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references.rs`
2019-09-25T21:35:47.7033678Z error: 1 errors occurred comparing output.
2019-09-25T21:35:47.7033799Z status: exit code: 1
2019-09-25T21:35:47.7033799Z status: exit code: 1
2019-09-25T21:35:47.7034618Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/auxiliary" "-A" "unused"
2019-09-25T21:35:47.7035167Z ------------------------------------------
2019-09-25T21:35:47.7035304Z 
2019-09-25T21:35:47.7035599Z ------------------------------------------
2019-09-25T21:35:47.7035798Z stderr:
2019-09-25T21:35:47.7035798Z stderr:
2019-09-25T21:35:47.7036102Z ------------------------------------------
2019-09-25T21:35:47.7036399Z warning: skipping const checks
2019-09-25T21:35:47.7039190Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:8:26
2019-09-25T21:35:47.7039673Z    |
2019-09-25T21:35:47.7039840Z LL | static FOO: &&mut u32 = &&mut 42;
2019-09-25T21:35:47.7040092Z 
2019-09-25T21:35:47.7040212Z warning: skipping const checks
2019-09-25T21:35:47.7040601Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:12:23
2019-09-25T21:35:47.7040786Z    |
2019-09-25T21:35:47.7040786Z    |
2019-09-25T21:35:47.7041025Z LL | static BAR: &mut () = &mut ();
2019-09-25T21:35:47.7041254Z 
2019-09-25T21:35:47.7041370Z warning: skipping const checks
2019-09-25T21:35:47.7041898Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:17:28
2019-09-25T21:35:47.7042084Z    |
2019-09-25T21:35:47.7042084Z    |
2019-09-25T21:35:47.7042391Z LL | static BOO: &mut Foo<()> = &mut Foo(());
2019-09-25T21:35:47.7042663Z 
2019-09-25T21:35:47.7042782Z warning: skipping const checks
2019-09-25T21:35:47.7043147Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:27:8
2019-09-25T21:35:47.7043454Z    |
2019-09-25T21:35:47.7043454Z    |
2019-09-25T21:35:47.7043675Z LL |     x: &UnsafeCell::new(42),
2019-09-25T21:35:47.7043813Z    |        ^^^^^^^^^^^^^^^^^^^^
2019-09-25T21:35:47.7043919Z 
2019-09-25T21:35:47.7044069Z warning: skipping const checks
2019-09-25T21:35:47.7044475Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:31:27
2019-09-25T21:35:47.7044648Z    |
2019-09-25T21:35:47.7044774Z LL | static OH_YES: &mut i32 = &mut 42;
2019-09-25T21:35:47.7045022Z 
2019-09-25T21:35:47.7045022Z 
2019-09-25T21:35:47.7045149Z error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-09-25T21:35:47.7045871Z    |
2019-09-25T21:35:47.7045871Z    |
2019-09-25T21:35:47.7046019Z LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-09-25T21:35:47.7046284Z 
2019-09-25T21:35:47.7046428Z error: aborting due to previous error
2019-09-25T21:35:47.7046533Z 
2019-09-25T21:35:47.7046633Z 
2019-09-25T21:35:47.7046633Z 
2019-09-25T21:35:47.7046938Z ------------------------------------------
2019-09-25T21:35:47.7047079Z 
2019-09-25T21:35:47.7047183Z 
2019-09-25T21:35:47.7047519Z ---- [ui] ui/consts/std/cell.rs stdout ----
2019-09-25T21:35:47.7047662Z 
2019-09-25T21:35:47.7047981Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-09-25T21:35:47.7048115Z status: exit code: 101
2019-09-25T21:35:47.7048857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/std/cell.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/cell" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/cell/auxiliary" "-A" "unused"
2019-09-25T21:35:47.7049748Z ------------------------------------------
2019-09-25T21:35:47.7049842Z 
2019-09-25T21:35:47.7055435Z ------------------------------------------
2019-09-25T21:35:47.7055494Z stderr:
2019-09-25T21:35:47.7055494Z stderr:
2019-09-25T21:35:47.7055873Z ------------------------------------------
2019-09-25T21:35:47.7055916Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-09-25T21:35:47.7056143Z    |
2019-09-25T21:35:47.7056143Z    |
2019-09-25T21:35:47.7056180Z LL | static FOO: Wrap<*mut u32> = Wrap(Cell::new(42).as_ptr());
2019-09-25T21:35:47.7056399Z 
2019-09-25T21:35:47.7056399Z 
2019-09-25T21:35:47.7056456Z [ERROR rustc_mir::transform::qualify_consts] old validator: []
2019-09-25T21:35:47.7056509Z [ERROR rustc_mir::transform::qualify_consts] new validator: [(/checkout/src/test/ui/consts/std/cell.rs:10:36: 10:42, "MutBorrow(Shared)")]
2019-09-25T21:35:47.7057447Z error: internal compiler error: src/librustc_mir/transform/qualify_consts.rs:1038: Disagreement between legacy and dataflow-based const validators.
2019-09-25T21:35:47.7058162Z     After filing an issue, use `-Zsuppress-const-validation-back-compat-ice` to compile your code.
2019-09-25T21:35:47.7058476Z    |
2019-09-25T21:35:47.7058476Z    |
2019-09-25T21:35:47.7058527Z LL | static FOO4: Wrap<*mut u32> = Wrap(FOO3.0.as_ptr());
2019-09-25T21:35:47.7058609Z 
2019-09-25T21:35:47.7058881Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
2019-09-25T21:35:47.7058958Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7058991Z 
2019-09-25T21:35:47.7058991Z 
2019-09-25T21:35:47.7059168Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-25T21:35:47.7059229Z 
2019-09-25T21:35:47.7060342Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-25T21:35:47.7060684Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-25T21:35:47.7060717Z 
2019-09-25T21:35:47.7060717Z 
2019-09-25T21:35:47.7061000Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-25T21:35:47.7061100Z error: aborting due to 2 previous errors
2019-09-25T21:35:47.7061131Z 
2019-09-25T21:35:47.7061379Z For more information about this error, try `rustc --explain E0492`.
2019-09-25T21:35:47.7061414Z 
2019-09-25T21:35:47.7061414Z 
2019-09-25T21:35:47.7061649Z ------------------------------------------
2019-09-25T21:35:47.7061681Z 
2019-09-25T21:35:47.7061722Z 
2019-09-25T21:35:47.7061970Z ---- [ui] ui/issues/issue-17718-static-unsafe-interior.rs stdout ----
2019-09-25T21:35:47.7062021Z 
2019-09-25T21:35:47.7062260Z error: test compilation failed although it shouldn't!
2019-09-25T21:35:47.7062313Z status: exit code: 101
2019-09-25T21:35:47.7063392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-static-unsafe-interior.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-unsafe-interior/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-unsafe-interior/auxiliary"
2019-09-25T21:35:47.7063675Z ------------------------------------------
2019-09-25T21:35:47.7063712Z 
2019-09-25T21:35:47.7063898Z ------------------------------------------
2019-09-25T21:35:47.7063936Z stderr:
2019-09-25T21:35:47.7063936Z stderr:
2019-09-25T21:35:47.7064129Z ------------------------------------------
2019-09-25T21:35:47.7064183Z [ERROR rustc_mir::transform::qualify_consts] old validator: []
2019-09-25T21:35:47.7064475Z [ERROR rustc_mir::transform::qualify_consts] new validator: [(/checkout/src/test/ui/issues/issue-17718-static-unsafe-interior.rs:37:48: 37:56, "MutBorrow(Shared)")]
2019-09-25T21:35:47.7064774Z error: internal compiler error: src/librustc_mir/transform/qualify_consts.rs:1038: Disagreement between legacy and dataflow-based const validators.
2019-09-25T21:35:47.7065005Z     After filing an issue, use `-Zsuppress-const-validation-back-compat-ice` to compile your code.
2019-09-25T21:35:47.7065277Z    |
2019-09-25T21:35:47.7065277Z    |
2019-09-25T21:35:47.7065647Z LL | static STATIC4: &'static MyUnsafePack<isize> = &STATIC2;
2019-09-25T21:35:47.7065873Z 
2019-09-25T21:35:47.7066120Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:871:9
2019-09-25T21:35:47.7066172Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7066216Z 
2019-09-25T21:35:47.7066216Z 
2019-09-25T21:35:47.7066256Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-25T21:35:47.7066282Z 
2019-09-25T21:35:47.7066579Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-25T21:35:47.7066823Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-25T21:35:47.7066852Z 
2019-09-25T21:35:47.7066852Z 
2019-09-25T21:35:47.7067106Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-25T21:35:47.7067176Z error: aborting due to previous error
2019-09-25T21:35:47.7067201Z 
2019-09-25T21:35:47.7067250Z 
2019-09-25T21:35:47.7067442Z ------------------------------------------
---
2019-09-25T21:35:47.7069077Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-25T21:35:47.7069141Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-25T21:35:47.7069177Z 
2019-09-25T21:35:47.7069199Z 
2019-09-25T21:35:47.7070933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-25T21:35:47.7071187Z 
2019-09-25T21:35:47.7071220Z 
2019-09-25T21:35:47.7075400Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-25T21:35:47.7075687Z Build completed unsuccessfully in 1:02:40
2019-09-25T21:35:47.7075687Z Build completed unsuccessfully in 1:02:40
2019-09-25T21:35:47.7121521Z == clock drift check ==
2019-09-25T21:35:47.7134744Z   local time: Wed Sep 25 21:35:47 UTC 2019
2019-09-25T21:35:47.8739006Z   network time: Wed, 25 Sep 2019 21:35:47 GMT
2019-09-25T21:35:47.8743219Z == end clock drift check ==
2019-09-25T21:35:49.1893889Z ##[error]Bash exited with code '1'.
2019-09-25T21:35:49.1938796Z ##[section]Starting: Checkout
2019-09-25T21:35:49.1941010Z ==============================================================================
2019-09-25T21:35:49.1941071Z Task         : Get sources
2019-09-25T21:35:49.1941120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

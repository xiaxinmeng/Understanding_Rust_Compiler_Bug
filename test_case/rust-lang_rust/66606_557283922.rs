plain
2019-11-21T20:31:00.1414672Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T20:31:00.1670439Z ##[command]git config gc.auto 0
2019-11-21T20:31:00.1755442Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T20:31:00.1800391Z ##[command]git config --get-all http.proxy
2019-11-21T20:31:00.1957713Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-21T21:32:10.6115642Z .............i...................................................................................... 4800/9264
2019-11-21T21:32:20.2580471Z .................................................................................................... 4900/9264
2019-11-21T21:32:25.2790664Z .................................................................................................... 5000/9264
2019-11-21T21:32:34.9842080Z .................................................................................................... 5100/9264
2019-11-21T21:32:40.1566048Z ...ii.ii...........i................................................................................ 5200/9264
2019-11-21T21:32:50.4744986Z .................................................................................................... 5400/9264
2019-11-21T21:33:00.9503882Z .....................................................................................i.............. 5500/9264
2019-11-21T21:33:09.2622942Z .................................................................................................... 5600/9264
2019-11-21T21:33:15.2425179Z .................................................................................................... 5700/9264
2019-11-21T21:33:15.2425179Z .................................................................................................... 5700/9264
2019-11-21T21:33:25.4423077Z .......................................................................ii...i..ii...........i....... 5800/9264
2019-11-21T21:33:47.4375728Z .................................................................................................... 6000/9264
2019-11-21T21:33:53.8943331Z .................................................................................................... 6100/9264
2019-11-21T21:33:53.8943331Z .................................................................................................... 6100/9264
2019-11-21T21:33:58.2126672Z ..............................................................................................i..ii. 6200/9264
2019-11-21T21:34:20.6689717Z .................................................................................................... 6400/9264
2019-11-21T21:34:29.9314235Z ..............................................................i..................................... 6500/9264
2019-11-21T21:34:32.2442444Z .................................................................................................... 6600/9264
2019-11-21T21:34:34.6544727Z ...................................................i................................................ 6700/9264
---
2019-11-21T21:39:10.9961469Z ---- [ui] ui/consts/miri_unleashed/mutable_const.rs stdout ----
2019-11-21T21:39:10.9961549Z diff of stderr:
2019-11-21T21:39:10.9961578Z 
2019-11-21T21:39:10.9961788Z - warning: skipping const checks
2019-11-21T21:39:10.9961840Z + error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-21T21:39:10.9962106Z 3    |
2019-11-21T21:39:10.9962106Z 3    |
2019-11-21T21:39:10.9962149Z 4 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-11-21T21:39:10.9962196Z 
2019-11-21T21:39:10.9962233Z 10 LL |         *MUTABLE_BEHIND_RAW = 99
2019-11-21T21:39:10.9962495Z 12 
2019-11-21T21:39:10.9962783Z - error: any use of this value will cause an error
2019-11-21T21:39:10.9962969Z -   --> $DIR/mutable_const.rs:15:9
2019-11-21T21:39:10.9963125Z -    |
2019-11-21T21:39:10.9963125Z -    |
2019-11-21T21:39:10.9963339Z - LL | / const MUTATING_BEHIND_RAW: () = {
2019-11-21T21:39:10.9963591Z - LL | |     // Test that `MUTABLE_BEHIND_RAW` is actually immutable, by doing this at const time.
2019-11-21T21:39:10.9963763Z - LL | |     unsafe {
2019-11-21T21:39:10.9963970Z - LL | |         *MUTABLE_BEHIND_RAW = 99
2019-11-21T21:39:10.9964191Z -    | |         ^^^^^^^^^^^^^^^^^^^^^^^^ tried to modify constant memory
2019-11-21T21:39:10.9964352Z - ...  |
2019-11-21T21:39:10.9964528Z - LL | |     }
2019-11-21T21:39:10.9964689Z - LL | | };
2019-11-21T21:39:10.9964849Z -    | |__-
2019-11-21T21:39:10.9965198Z - note: lint level defined here
2019-11-21T21:39:10.9965377Z -   --> $DIR/mutable_const.rs:4:9
2019-11-21T21:39:10.9965532Z -    |
2019-11-21T21:39:10.9965728Z - LL | #![deny(const_err)]
---
2019-11-21T21:39:10.9966685Z 
2019-11-21T21:39:10.9966711Z 
2019-11-21T21:39:10.9966754Z The actual stderr differed from the expected stderr.
2019-11-21T21:39:10.9967088Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/mutable_const.stderr
2019-11-21T21:39:10.9967354Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T21:39:10.9967620Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const.rs`
2019-11-21T21:39:10.9967834Z error: 1 errors occurred comparing output.
2019-11-21T21:39:10.9967879Z status: exit code: 1
2019-11-21T21:39:10.9967879Z status: exit code: 1
2019-11-21T21:39:10.9968713Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/auxiliary" "-A" "unused"
2019-11-21T21:39:10.9969056Z ------------------------------------------
2019-11-21T21:39:10.9969110Z 
2019-11-21T21:39:10.9969327Z ------------------------------------------
2019-11-21T21:39:10.9969372Z stderr:
2019-11-21T21:39:10.9969372Z stderr:
2019-11-21T21:39:10.9969700Z ------------------------------------------
2019-11-21T21:39:10.9969888Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-21T21:39:10.9970194Z    |
2019-11-21T21:39:10.9970194Z    |
2019-11-21T21:39:10.9970246Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-11-21T21:39:10.9970324Z 
2019-11-21T21:39:10.9970379Z warning: skipping const checks
2019-11-21T21:39:10.9970611Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:15:9
2019-11-21T21:39:10.9970656Z    |
2019-11-21T21:39:10.9970656Z    |
2019-11-21T21:39:10.9970719Z LL |         *MUTABLE_BEHIND_RAW = 99 //~ WARN skipping const checks
2019-11-21T21:39:10.9970790Z 
2019-11-21T21:39:10.9970846Z error: aborting due to previous error
2019-11-21T21:39:10.9970872Z 
2019-11-21T21:39:10.9971280Z For more information about this error, try `rustc --explain E0492`.
2019-11-21T21:39:10.9971280Z For more information about this error, try `rustc --explain E0492`.
2019-11-21T21:39:10.9971316Z 
2019-11-21T21:39:10.9971530Z ------------------------------------------
2019-11-21T21:39:10.9971579Z 
2019-11-21T21:39:10.9971600Z 
2019-11-21T21:39:10.9971810Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2019-11-21T21:39:10.9971848Z 
2019-11-21T21:39:10.9971908Z error: Error: expected failure status (Some(101)) but received status Some(1).
2019-11-21T21:39:10.9971949Z status: exit code: 1
2019-11-21T21:39:10.9972694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2019-11-21T21:39:10.9972989Z ------------------------------------------
2019-11-21T21:39:10.9973115Z 
2019-11-21T21:39:10.9973329Z ------------------------------------------
2019-11-21T21:39:10.9973369Z stderr:
2019-11-21T21:39:10.9973369Z stderr:
2019-11-21T21:39:10.9973572Z ------------------------------------------
2019-11-21T21:39:10.9973622Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-21T21:39:10.9973920Z    |
2019-11-21T21:39:10.9973920Z    |
2019-11-21T21:39:10.9973960Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2019-11-21T21:39:10.9974024Z 
2019-11-21T21:39:10.9974079Z error: aborting due to previous error
2019-11-21T21:39:10.9974104Z 
2019-11-21T21:39:10.9974323Z For more information about this error, try `rustc --explain E0492`.
---
2019-11-21T21:39:10.9975079Z - warning: skipping const checks
2019-11-21T21:39:10.9975142Z + error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9975335Z 2   --> $DIR/mutable_references.rs:8:26
2019-11-21T21:39:10.9975374Z 3    |
2019-11-21T21:39:10.9975412Z 4 LL | static FOO: &&mut u32 = &&mut 42;
2019-11-21T21:39:10.9975639Z -    |                          ^^^^^^^
2019-11-21T21:39:10.9975685Z +    |                          ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9975742Z 6 
2019-11-21T21:39:10.9975926Z - warning: skipping const checks
2019-11-21T21:39:10.9975926Z - warning: skipping const checks
2019-11-21T21:39:10.9975972Z + error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9976160Z 8   --> $DIR/mutable_references.rs:11:23
2019-11-21T21:39:10.9976217Z 9    |
2019-11-21T21:39:10.9976260Z 10 LL | static BAR: &mut () = &mut ();
2019-11-21T21:39:10.9976484Z -    |                       ^^^^^^^
2019-11-21T21:39:10.9976529Z +    |                       ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9976567Z 12 
2019-11-21T21:39:10.9976744Z - warning: skipping const checks
2019-11-21T21:39:10.9976744Z - warning: skipping const checks
2019-11-21T21:39:10.9976806Z + error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9976995Z 14   --> $DIR/mutable_references.rs:16:28
2019-11-21T21:39:10.9977034Z 15    |
2019-11-21T21:39:10.9977091Z 16 LL | static BOO: &mut Foo<()> = &mut Foo(());
2019-11-21T21:39:10.9977303Z -    |                            ^^^^^^^^^^^^
2019-11-21T21:39:10.9977444Z +    |                            ^^^^^^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9977489Z 18 
2019-11-21T21:39:10.9977690Z - warning: skipping const checks
2019-11-21T21:39:10.9977690Z - warning: skipping const checks
2019-11-21T21:39:10.9977740Z + error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-21T21:39:10.9978002Z 21    |
2019-11-21T21:39:10.9978038Z 22 LL |     x: &UnsafeCell::new(42),
2019-11-21T21:39:10.9978080Z 
2019-11-21T21:39:10.9978116Z 23    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-21T21:39:10.9978116Z 23    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-21T21:39:10.9978151Z 24 
2019-11-21T21:39:10.9978328Z - warning: skipping const checks
2019-11-21T21:39:10.9978391Z + error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9978581Z 26   --> $DIR/mutable_references.rs:30:27
2019-11-21T21:39:10.9978621Z 27    |
2019-11-21T21:39:10.9978676Z 28 LL | static OH_YES: &mut i32 = &mut 42;
2019-11-21T21:39:10.9978890Z -    |                           ^^^^^^^
2019-11-21T21:39:10.9978935Z +    |                           ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9978990Z 30 
2019-11-21T21:39:10.9978990Z 30 
2019-11-21T21:39:10.9979033Z 31 error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-21T21:39:10.9979391Z 
2019-11-21T21:39:10.9979391Z 
2019-11-21T21:39:10.9979428Z 34 LL |     *OH_YES = 99;
2019-11-21T21:39:10.9979520Z 36 
2019-11-21T21:39:10.9979703Z - error: aborting due to previous error
2019-11-21T21:39:10.9979745Z + error: aborting due to 6 previous errors
2019-11-21T21:39:10.9979780Z 38 
2019-11-21T21:39:10.9979780Z 38 
2019-11-21T21:39:10.9980007Z - For more information about this error, try `rustc --explain E0594`.
2019-11-21T21:39:10.9980054Z + Some errors have detailed explanations: E0017, E0492, E0594.
2019-11-21T21:39:10.9980263Z + For more information about an error, try `rustc --explain E0017`.
2019-11-21T21:39:10.9980330Z 40 
2019-11-21T21:39:10.9980353Z 
2019-11-21T21:39:10.9980375Z 
2019-11-21T21:39:10.9980413Z The actual stderr differed from the expected stderr.
2019-11-21T21:39:10.9980720Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
2019-11-21T21:39:10.9980946Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T21:39:10.9981190Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references.rs`
2019-11-21T21:39:10.9981279Z error: 1 errors occurred comparing output.
2019-11-21T21:39:10.9981316Z status: exit code: 1
2019-11-21T21:39:10.9981316Z status: exit code: 1
2019-11-21T21:39:10.9982062Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/auxiliary" "-A" "unused"
2019-11-21T21:39:10.9982365Z ------------------------------------------
2019-11-21T21:39:10.9982394Z 
2019-11-21T21:39:10.9982579Z ------------------------------------------
2019-11-21T21:39:10.9982636Z stderr:
2019-11-21T21:39:10.9982636Z stderr:
2019-11-21T21:39:10.9982820Z ------------------------------------------
2019-11-21T21:39:10.9982865Z error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9983101Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:8:26
2019-11-21T21:39:10.9983144Z    |
2019-11-21T21:39:10.9983181Z LL | static FOO: &&mut u32 = &&mut 42;
2019-11-21T21:39:10.9983311Z    |                          ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9983384Z error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9983632Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:11:23
2019-11-21T21:39:10.9983702Z    |
2019-11-21T21:39:10.9983702Z    |
2019-11-21T21:39:10.9983739Z LL | static BAR: &mut () = &mut ();
2019-11-21T21:39:10.9983780Z    |                       ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9983863Z error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9984088Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:16:28
2019-11-21T21:39:10.9984130Z    |
2019-11-21T21:39:10.9984130Z    |
2019-11-21T21:39:10.9984186Z LL | static BOO: &mut Foo<()> = &mut Foo(());
2019-11-21T21:39:10.9984231Z    |                            ^^^^^^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9984258Z 
2019-11-21T21:39:10.9984325Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-21T21:39:10.9984598Z    |
2019-11-21T21:39:10.9984736Z LL |     x: &UnsafeCell::new(42),
2019-11-21T21:39:10.9984774Z    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-21T21:39:10.9984800Z 
2019-11-21T21:39:10.9984800Z 
2019-11-21T21:39:10.9984838Z error[E0017]: references in statics may only refer to immutable values
2019-11-21T21:39:10.9985107Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:30:27
2019-11-21T21:39:10.9985150Z    |
2019-11-21T21:39:10.9985186Z LL | static OH_YES: &mut i32 = &mut 42;
2019-11-21T21:39:10.9985246Z    |                           ^^^^^^^ statics require immutable values
2019-11-21T21:39:10.9985274Z 
2019-11-21T21:39:10.9985315Z error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-21T21:39:10.9985606Z    |
2019-11-21T21:39:10.9985606Z    |
2019-11-21T21:39:10.9985651Z LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-21T21:39:10.9985743Z 
2019-11-21T21:39:10.9985779Z error: aborting due to 6 previous errors
2019-11-21T21:39:10.9985804Z 
2019-11-21T21:39:10.9985842Z Some errors have detailed explanations: E0017, E0492, E0594.
---
2019-11-21T21:39:11.0006365Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-21T21:39:11.0006447Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-21T21:39:11.0022694Z 
2019-11-21T21:39:11.0022781Z 
2019-11-21T21:39:11.0024864Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-21T21:39:11.0025184Z 
2019-11-21T21:39:11.0025216Z 
2019-11-21T21:39:11.0032749Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-21T21:39:11.0032903Z Build completed unsuccessfully in 1:02:03
2019-11-21T21:39:11.0032903Z Build completed unsuccessfully in 1:02:03
2019-11-21T21:39:11.0098327Z == clock drift check ==
2019-11-21T21:39:11.0117371Z   local time: Thu Nov 21 21:39:11 UTC 2019
2019-11-21T21:39:11.1506496Z   network time: Thu, 21 Nov 2019 21:39:11 GMT
2019-11-21T21:39:11.1506885Z == end clock drift check ==
2019-11-21T21:39:12.0028194Z 
2019-11-21T21:39:12.0149972Z ##[error]Bash exited with code '1'.
2019-11-21T21:39:12.0193993Z ##[section]Starting: Checkout
2019-11-21T21:39:12.0195908Z ==============================================================================
2019-11-21T21:39:12.0195968Z Task         : Get sources
2019-11-21T21:39:12.0196021Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

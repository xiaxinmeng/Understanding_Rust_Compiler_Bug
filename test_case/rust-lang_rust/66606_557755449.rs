plain
2019-11-23T01:00:22.1304366Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T01:00:22.1466494Z ##[command]git config gc.auto 0
2019-11-23T01:00:22.1541712Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T01:00:22.1602757Z ##[command]git config --get-all http.proxy
2019-11-23T01:00:22.1737724Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-23T01:59:44.8863339Z .............i...................................................................................... 4800/9264
2019-11-23T01:59:54.1417972Z .................................................................................................... 4900/9264
2019-11-23T01:59:58.9383284Z .................................................................................................... 5000/9264
2019-11-23T02:00:08.4462329Z .................................................................................................... 5100/9264
2019-11-23T02:00:13.7258432Z ...ii.ii...........i................................................................................ 5200/9264
2019-11-23T02:00:23.6017425Z .................................................................................................... 5400/9264
2019-11-23T02:00:33.6001586Z .....................................................................................i.............. 5500/9264
2019-11-23T02:00:41.5053946Z .................................................................................................... 5600/9264
2019-11-23T02:00:47.2557541Z .................................................................................................... 5700/9264
2019-11-23T02:00:47.2557541Z .................................................................................................... 5700/9264
2019-11-23T02:00:57.2284873Z .......................................................................ii...i..ii............i...... 5800/9264
2019-11-23T02:01:19.3132348Z .................................................................................................... 6000/9264
2019-11-23T02:01:24.2887672Z .................................................................................................... 6100/9264
2019-11-23T02:01:24.2887672Z .................................................................................................... 6100/9264
2019-11-23T02:01:28.3314024Z ..............................................................................................i..ii. 6200/9264
2019-11-23T02:01:50.4840610Z .................................................................................................... 6400/9264
2019-11-23T02:01:59.4018674Z ...............................................................i.................................... 6500/9264
2019-11-23T02:02:01.4765620Z .................................................................................................... 6600/9264
2019-11-23T02:02:03.6604600Z ...................................................i................................................ 6700/9264
---
2019-11-23T02:06:37.2972561Z ---- [ui] ui/consts/miri_unleashed/mutable_const.rs stdout ----
2019-11-23T02:06:37.2972621Z diff of stderr:
2019-11-23T02:06:37.2972657Z 
2019-11-23T02:06:37.2972907Z - warning: skipping const checks
2019-11-23T02:06:37.2972979Z + error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-23T02:06:37.2973283Z 3    |
2019-11-23T02:06:37.2973283Z 3    |
2019-11-23T02:06:37.2973334Z 4 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-11-23T02:06:37.2973432Z 5    |                                      ^^^^^^^^^^^^^^^^^^^^
2019-11-23T02:06:37.2973477Z 6 
2019-11-23T02:06:37.2973692Z - warning: skipping const checks
2019-11-23T02:06:37.2973755Z + error[E0019]: constant contains unimplemented expression type
2019-11-23T02:06:37.2973755Z + error[E0019]: constant contains unimplemented expression type
2019-11-23T02:06:37.2973993Z 8   --> $DIR/mutable_const.rs:15:9
2019-11-23T02:06:37.2974039Z 9    |
2019-11-23T02:06:37.2974083Z 10 LL |         *MUTABLE_BEHIND_RAW = 99
2019-11-23T02:06:37.2974173Z 11    |         ^^^^^^^^^^^^^^^^^^^^^^^^
2019-11-23T02:06:37.2974214Z 12 
2019-11-23T02:06:37.2974442Z - error: any use of this value will cause an error
2019-11-23T02:06:37.2974682Z -   --> $DIR/mutable_const.rs:15:9
2019-11-23T02:06:37.2974682Z -   --> $DIR/mutable_const.rs:15:9
2019-11-23T02:06:37.2975287Z -    |
2019-11-23T02:06:37.2975641Z - LL | / const MUTATING_BEHIND_RAW: () = {
2019-11-23T02:06:37.2976055Z - LL | |     // Test that `MUTABLE_BEHIND_RAW` is actually immutable, by doing this at const time.
2019-11-23T02:06:37.2976211Z - LL | |     unsafe {
2019-11-23T02:06:37.2976376Z - LL | |         *MUTABLE_BEHIND_RAW = 99
2019-11-23T02:06:37.2976587Z -    | |         ^^^^^^^^^^^^^^^^^^^^^^^^ tried to modify constant memory
2019-11-23T02:06:37.2976739Z - ...  |
2019-11-23T02:06:37.2976878Z - LL | |     }
2019-11-23T02:06:37.2977015Z - LL | | };
2019-11-23T02:06:37.2977177Z -    | |__-
2019-11-23T02:06:37.2977463Z - note: lint level defined here
2019-11-23T02:06:37.2977644Z -   --> $DIR/mutable_const.rs:4:9
2019-11-23T02:06:37.2977778Z -    |
2019-11-23T02:06:37.2977925Z - LL | #![deny(const_err)]
---
2019-11-23T02:06:37.2978963Z 
2019-11-23T02:06:37.2978983Z 
2019-11-23T02:06:37.2979018Z The actual stderr differed from the expected stderr.
2019-11-23T02:06:37.2979291Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/mutable_const.stderr
2019-11-23T02:06:37.2979485Z To update references, rerun the tests and pass the `--bless` flag
2019-11-23T02:06:37.2979702Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const.rs`
2019-11-23T02:06:37.2979781Z error: 1 errors occurred comparing output.
2019-11-23T02:06:37.2979823Z status: exit code: 1
2019-11-23T02:06:37.2979823Z status: exit code: 1
2019-11-23T02:06:37.2980664Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/auxiliary" "-A" "unused"
2019-11-23T02:06:37.2981628Z ------------------------------------------
2019-11-23T02:06:37.2981659Z 
2019-11-23T02:06:37.2982028Z ------------------------------------------
2019-11-23T02:06:37.2982088Z stderr:
2019-11-23T02:06:37.2982088Z stderr:
2019-11-23T02:06:37.2982279Z ------------------------------------------
2019-11-23T02:06:37.2982339Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-23T02:06:37.2982634Z    |
2019-11-23T02:06:37.2982634Z    |
2019-11-23T02:06:37.2982678Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-11-23T02:06:37.2982771Z 
2019-11-23T02:06:37.2982812Z error[E0019]: constant contains unimplemented expression type
2019-11-23T02:06:37.2983543Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:15:9
2019-11-23T02:06:37.2983613Z    |
2019-11-23T02:06:37.2983613Z    |
2019-11-23T02:06:37.2983659Z LL |         *MUTABLE_BEHIND_RAW = 99 //~ WARN skipping const checks
2019-11-23T02:06:37.2983737Z 
2019-11-23T02:06:37.2983799Z error: aborting due to 2 previous errors
2019-11-23T02:06:37.2983827Z 
2019-11-23T02:06:37.2983881Z Some errors have detailed explanations: E0019, E0492.
---
2019-11-23T02:06:37.2984995Z - warning: skipping const checks
2019-11-23T02:06:37.2985057Z + error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2985275Z 2   --> $DIR/mutable_references.rs:8:26
2019-11-23T02:06:37.2985319Z 3    |
2019-11-23T02:06:37.2985382Z 4 LL | static FOO: &&mut u32 = &&mut 42;
2019-11-23T02:06:37.2985618Z -    |                          ^^^^^^^
2019-11-23T02:06:37.2985789Z +    |                          ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2985843Z 6 
2019-11-23T02:06:37.2986070Z - warning: skipping const checks
2019-11-23T02:06:37.2986070Z - warning: skipping const checks
2019-11-23T02:06:37.2986121Z + error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2986520Z 8   --> $DIR/mutable_references.rs:11:23
2019-11-23T02:06:37.2986559Z 9    |
2019-11-23T02:06:37.2986762Z 10 LL | static BAR: &mut () = &mut ();
2019-11-23T02:06:37.2986974Z -    |                       ^^^^^^^
2019-11-23T02:06:37.2987016Z +    |                       ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2987078Z 12 
2019-11-23T02:06:37.2987247Z - warning: skipping const checks
2019-11-23T02:06:37.2987247Z - warning: skipping const checks
2019-11-23T02:06:37.2987289Z + error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2987465Z 14   --> $DIR/mutable_references.rs:16:28
2019-11-23T02:06:37.2987519Z 15    |
2019-11-23T02:06:37.2987556Z 16 LL | static BOO: &mut Foo<()> = &mut Foo(());
2019-11-23T02:06:37.2987782Z -    |                            ^^^^^^^^^^^^
2019-11-23T02:06:37.2987827Z +    |                            ^^^^^^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2987863Z 18 
2019-11-23T02:06:37.2988029Z - warning: skipping const checks
2019-11-23T02:06:37.2988029Z - warning: skipping const checks
2019-11-23T02:06:37.2988092Z + error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-23T02:06:37.2988307Z 21    |
2019-11-23T02:06:37.2988360Z 22 LL |     x: &UnsafeCell::new(42),
2019-11-23T02:06:37.2988455Z 
2019-11-23T02:06:37.2988495Z 23    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-23T02:06:37.2988495Z 23    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-23T02:06:37.2988529Z 24 
2019-11-23T02:06:37.2988736Z - warning: skipping const checks
2019-11-23T02:06:37.2988779Z + error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2988956Z 26   --> $DIR/mutable_references.rs:30:27
2019-11-23T02:06:37.2989013Z 27    |
2019-11-23T02:06:37.2989049Z 28 LL | static OH_YES: &mut i32 = &mut 42;
2019-11-23T02:06:37.2989253Z -    |                           ^^^^^^^
2019-11-23T02:06:37.2989313Z +    |                           ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2989349Z 30 
2019-11-23T02:06:37.2989349Z 30 
2019-11-23T02:06:37.2989389Z 31 error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-23T02:06:37.2989611Z 
2019-11-23T02:06:37.2989611Z 
2019-11-23T02:06:37.2989645Z 34 LL |     *OH_YES = 99;
2019-11-23T02:06:37.2989744Z 36 
2019-11-23T02:06:37.2989916Z - error: aborting due to previous error
2019-11-23T02:06:37.2989956Z + error: aborting due to 6 previous errors
2019-11-23T02:06:37.2990007Z 38 
2019-11-23T02:06:37.2990007Z 38 
2019-11-23T02:06:37.2990206Z - For more information about this error, try `rustc --explain E0594`.
2019-11-23T02:06:37.2990250Z + Some errors have detailed explanations: E0017, E0492, E0594.
2019-11-23T02:06:37.2990471Z + For more information about an error, try `rustc --explain E0017`.
2019-11-23T02:06:37.2990509Z 40 
2019-11-23T02:06:37.2990531Z 
2019-11-23T02:06:37.2990552Z 
2019-11-23T02:06:37.2990606Z The actual stderr differed from the expected stderr.
2019-11-23T02:06:37.2990875Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
2019-11-23T02:06:37.2991080Z To update references, rerun the tests and pass the `--bless` flag
2019-11-23T02:06:37.2991569Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references.rs`
2019-11-23T02:06:37.2991643Z error: 1 errors occurred comparing output.
2019-11-23T02:06:37.2991699Z status: exit code: 1
2019-11-23T02:06:37.2991699Z status: exit code: 1
2019-11-23T02:06:37.2993004Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/auxiliary" "-A" "unused"
2019-11-23T02:06:37.2993377Z ------------------------------------------
2019-11-23T02:06:37.2993420Z 
2019-11-23T02:06:37.2993659Z ------------------------------------------
2019-11-23T02:06:37.2993705Z stderr:
2019-11-23T02:06:37.2993705Z stderr:
2019-11-23T02:06:37.2993914Z ------------------------------------------
2019-11-23T02:06:37.2993984Z error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2994238Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:8:26
2019-11-23T02:06:37.2994288Z    |
2019-11-23T02:06:37.2994358Z LL | static FOO: &&mut u32 = &&mut 42;
2019-11-23T02:06:37.2994410Z    |                          ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2994487Z error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2994767Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:11:23
2019-11-23T02:06:37.2994816Z    |
2019-11-23T02:06:37.2994816Z    |
2019-11-23T02:06:37.2994860Z LL | static BAR: &mut () = &mut ();
2019-11-23T02:06:37.2994928Z    |                       ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2995109Z error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2995544Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:16:28
2019-11-23T02:06:37.2995769Z    |
2019-11-23T02:06:37.2995769Z    |
2019-11-23T02:06:37.2995806Z LL | static BOO: &mut Foo<()> = &mut Foo(());
2019-11-23T02:06:37.2995850Z    |                            ^^^^^^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2995902Z 
2019-11-23T02:06:37.2996107Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-23T02:06:37.2996541Z    |
2019-11-23T02:06:37.2996575Z LL |     x: &UnsafeCell::new(42),
2019-11-23T02:06:37.2996611Z    |        ^^^^^^^^^^^^^^^^^^^^
2019-11-23T02:06:37.2996633Z 
2019-11-23T02:06:37.2996633Z 
2019-11-23T02:06:37.2996688Z error[E0017]: references in statics may only refer to immutable values
2019-11-23T02:06:37.2996904Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:30:27
2019-11-23T02:06:37.2997140Z    |
2019-11-23T02:06:37.2997195Z LL | static OH_YES: &mut i32 = &mut 42;
2019-11-23T02:06:37.2997235Z    |                           ^^^^^^^ statics require immutable values
2019-11-23T02:06:37.2997261Z 
2019-11-23T02:06:37.2997300Z error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-23T02:06:37.2997573Z    |
2019-11-23T02:06:37.2997573Z    |
2019-11-23T02:06:37.2997615Z LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2019-11-23T02:06:37.2997698Z 
2019-11-23T02:06:37.2997733Z error: aborting due to 6 previous errors
2019-11-23T02:06:37.2997756Z 
2019-11-23T02:06:37.2997810Z Some errors have detailed explanations: E0017, E0492, E0594.
2019-11-23T02:06:37.2997810Z Some errors have detailed explanations: E0017, E0492, E0594.
2019-11-23T02:06:37.2998018Z For more information about an error, try `rustc --explain E0017`.
2019-11-23T02:06:37.2998045Z 
2019-11-23T02:06:37.2998236Z ------------------------------------------
2019-11-23T02:06:37.2998261Z 
2019-11-23T02:06:37.2998282Z 
2019-11-23T02:06:37.2998478Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2019-11-23T02:06:37.2998505Z 
2019-11-23T02:06:37.2998630Z error: Error: expected failure status (Some(101)) but received status Some(1).
2019-11-23T02:06:37.2998674Z status: exit code: 1
2019-11-23T02:06:37.2999410Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2019-11-23T02:06:37.2999692Z ------------------------------------------
2019-11-23T02:06:37.2999882Z 
2019-11-23T02:06:37.3000057Z ------------------------------------------
2019-11-23T02:06:37.3000093Z stderr:
2019-11-23T02:06:37.3000093Z stderr:
2019-11-23T02:06:37.3000456Z ------------------------------------------
2019-11-23T02:06:37.3000500Z error[E0492]: cannot borrow a constant which may contain interior mutability, create a static instead
2019-11-23T02:06:37.3000760Z    |
2019-11-23T02:06:37.3000760Z    |
2019-11-23T02:06:37.3000796Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2019-11-23T02:06:37.3000871Z 
2019-11-23T02:06:37.3000971Z error: aborting due to previous error
2019-11-23T02:06:37.3000993Z 
2019-11-23T02:06:37.3001203Z For more information about this error, try `rustc --explain E0492`.
---
2019-11-23T02:06:37.3009009Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-23T02:06:37.3009071Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-23T02:06:37.3019840Z 
2019-11-23T02:06:37.3020134Z 
2019-11-23T02:06:37.3022220Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-23T02:06:37.3022896Z 
2019-11-23T02:06:37.3022942Z 
2019-11-23T02:06:37.3026644Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-23T02:06:37.3026820Z Build completed unsuccessfully in 1:00:25
2019-11-23T02:06:37.3026820Z Build completed unsuccessfully in 1:00:25
2019-11-23T02:06:37.3084735Z == clock drift check ==
2019-11-23T02:06:37.3102380Z   local time: Sat Nov 23 02:06:37 UTC 2019
2019-11-23T02:06:37.8448598Z   network time: Sat, 23 Nov 2019 02:06:37 GMT
2019-11-23T02:06:37.8453153Z == end clock drift check ==
2019-11-23T02:06:38.6107346Z 
2019-11-23T02:06:38.6227116Z ##[error]Bash exited with code '1'.
2019-11-23T02:06:38.6269005Z ##[section]Starting: Checkout
2019-11-23T02:06:38.6270267Z ==============================================================================
2019-11-23T02:06:38.6270310Z Task         : Get sources
2019-11-23T02:06:38.6270346Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

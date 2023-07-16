plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4e45b50f-0b1e-444a-ab58-18b805449ab9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71631/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71631/merge:refs/remotes/pull/71631/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................................................................................................... 1500/9963
............FFF..................................................................................... 1600/9963
.................................................................................................... 1700/9963
..............i..................................................................................... 1800/9963
...........................F..F......F...FF......................................................... 1900/9963
.................................................................................................... 2100/9963
.................................................................................................... 2100/9963
....................iiiii........................................................................... 2200/9963
.................................................................................................... 2400/9963
.................................................................................................... 2500/9963
.................................................................................................... 2600/9963
.................................................................................................... 2700/9963
---
....i...............i............................................................................... 5100/9963
.................................................................................................... 5200/9963
..................................................i................................................. 5300/9963
.........................................i.......................................................... 5400/9963
...........................................ii.ii........i...i....................................... 5500/9963
..........................................................................................i......... 5700/9963
.................................................................................................... 5800/9963
.........................ii.....................................i................................... 5900/9963
.................................................................................................... 6000/9963
.................................................................................................... 6000/9963
.................................................................................................... 6100/9963
...........................................................ii...i..ii...........i................... 6200/9963
.................................................................................................... 6400/9963
.................................................................................................... 6500/9963
.................................................................................................... 6500/9963
...........................................................................................i..ii.... 6600/9963
.................................................................................................... 6800/9963
............................................................................................i....... 6900/9963
.................................................................................................... 7000/9963
.................................................................................................... 7100/9963
---
.................................................................................................... 7900/9963
.................................................................................................... 8000/9963
...............................F.................................................................... 8100/9963
..i................................................................................................. 8200/9963
...................................................iiiiii.iiiii.i................................... 8300/9963
....i............................................................................................... 8500/9963
.................................................................................................... 8600/9963
.................................................................................................... 8700/9963
.................................................................................................... 8800/9963
---
---- [ui] ui/consts/const-eval/const_fn_ptr_fail.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs:9:1
   |
LL | / const fn bar(x: usize) -> usize { //~ WARNING skipping const checks
LL | |     X(x) // FIXME: this should error someday
   | |_^

warning: 1 warning emitted


error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs:10:5
   |
LL |     X(x) // FIXME: this should error someday


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/const-eval/const_fn_ptr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:11:1
   |
LL | / const fn bar(x: usize) -> usize { //~ WARNING skipping const checks
LL | |     X(x)
   | |_^

warning: skipping const checks
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:15:1
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:15:1
   |
LL | / const fn bar_const(x: usize) -> usize { //~ WARNING skipping const checks
LL | |     X_CONST(x)
   | |_^

warning: skipping const checks
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:19:1
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:19:1
   |
LL | / const fn foo(x: fn(usize) -> usize, y: usize)  -> usize { //~ WARNING skipping const checks
LL | |     x(y)
   | |_^

warning: 3 warnings emitted

---

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:16:5
   |
LL |     X_CONST(x)

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr.rs:20:5
   |
   |
LL |     x(y)
   |     ^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/const-eval/const_fn_ptr_fail2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:12:1
   |
LL | / const fn bar(x: fn(usize) -> usize, y: usize) -> usize { //~ WARN skipping const checks
LL | |     x(y)
   | |_^

warning: 1 warning emitted


error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:13:5
   |
LL |     x(y)
   |     ^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/miri_unleashed/assoc_const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:14:5
   |
LL |     const F: u32 = (U::X, 42).1; //~ WARN skipping const checks

warning: 1 warning emitted

error: internal compiler error: skipping const checks is for testing error paths only
error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:14:20
   |
LL |     const F: u32 = (U::X, 42).1; //~ WARN skipping const checks


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/miri_unleashed/const_refers_to_static.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-Zdeduplicate-diagnostics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:11:1
   |
LL | / const MUTATE_INTERIOR_MUT: usize = {
LL | | //~^ WARN skipping const checks
LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
LL | |     FOO.fetch_add(1, Ordering::Relaxed)
   | |__^

warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:17:1
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:17:1
   |
LL | / const READ_INTERIOR_MUT: usize = {
LL | | //~^ WARN skipping const checks
LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
LL | |     unsafe { *(&FOO as *const _ as *const usize) }
   | |__^

warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:24:1
---

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:14:5
   |
LL |     FOO.fetch_add(1, Ordering::Relaxed)

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:14:5
   |
   |
LL |     FOO.fetch_add(1, Ordering::Relaxed)

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:20:17
   |
   |
LL |     unsafe { *(&FOO as *const _ as *const usize) }

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:20:14
   |
   |
LL |     unsafe { *(&FOO as *const _ as *const usize) }

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:24:32
   |
   |
LL | const READ_MUT: u32 = unsafe { MUTABLE };
   |                                ^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -Z deduplicate-diagnostics -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------


---
8 
+ error: internal compiler error: skipping const checks is for testing error paths only
+   --> $DIR/mutable_const2.rs:13:38
+    |
+ LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
+ 
9 error: internal compiler error: mutable allocation in constant
10   --> $DIR/mutable_const2.rs:13:1
11    |
11    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:13:1
   |
LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;

warning: 1 warning emitted

error: internal compiler error: skipping const checks is for testing error paths only
error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:13:38
   |
LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;

error: internal compiler error: mutable allocation in constant
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:13:1
   |
   |
LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/miri_unleashed/read_from_static.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/read_from_static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/read_from_static/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/read_from_static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/read_from_static.rs:5:1
   |
LL | static OH_YES: &mut i32 = &mut 42;

warning: 1 warning emitted

error: internal compiler error: skipping const checks is for testing error paths only
error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/read_from_static.rs:5:27
   |
LL | static OH_YES: &mut i32 = &mut 42;


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------



---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 4
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:21:1
   |
LL | / const MUH: Meh = Meh { //~ WARN skipping const checks
LL | |     x: &UnsafeCell::new(42),
   | |__^

thread 'rustc' panicked at 'assertion failed: `(left != right)`
  left: `Const`,
  left: `Const`,
 right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:174:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
warning: 1 warning emitted

error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
   |
LL |     x: &UnsafeCell::new(42),
   |        ^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17
stack backtrace:
   0:     0x7f9c0b3287a6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb3fb89ef6446e958
   1:     0x7f9c0b363c9c - core::fmt::write::h8582f0e548ae2670
   2:     0x7f9c0b319f23 - std::io::Write::write_fmt::h3f2bcd12bebd6108
   3:     0x7f9c0b32d8b2 - std::panicking::default_hook::{{closure}}::hec573baed92d7dcc
   4:     0x7f9c0b32d60f - std::panicking::default_hook::h2594756b6ad8235f
   5:     0x7f9c0b8b6fec - rustc_driver::report_ice::h1de2ed4d8a157875
   6:     0x7f9c0b32e038 - std::panicking::rust_panic_with_hook::h070f5ca1752a2168
   7:     0x7f9c0daac5de - std::panicking::begin_panic::h0545b4207f19b875
   8:     0x7f9c0dadb794 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h74146e1cd7f5b34c
   9:     0x7f9c0b8d6852 - core::ptr::drop_in_place::h4730c647290b74ab
  10:     0x7f9c0b8dac3c - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h6c21c79101f5bbcd
  11:     0x7f9c0b9bb7ac - core::ptr::drop_in_place::haa7381c54f36b4df
  12:     0x7f9c0b9a5331 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hb023c2442f2a044f
  13:     0x7f9c0b8bddbb - scoped_tls::ScopedKey<T>::set::ha34d629571d0c606
  14:     0x7f9c0b8bd5f2 - rustc_ast::attr::with_globals::hf2d1df3cb5edc365
  15:     0x7f9c0b8c75fe - std::sys_common::backtrace::__rust_begin_short_backtrace::h5213ed7e70a7e6f0
  16:     0x7f9c0b8c2b56 - std::panicking::try::h0084ae37832b3ae3
  17:     0x7f9c0b9a648e - core::ops::function::FnOnce::call_once{{vtable.shim}}::h228c769375656a2e
  18:     0x7f9c0b33e2ba - std::sys::unix::thread::Thread::new::thread_start::hf27560bbc9e7b5d4
  19:     0x7f9c06ad66db - start_thread
  20:     0x7f9c0afe988f - __clone
  21:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
thread panicked while panicking. aborting.

------------------------------------------



---- [ui] ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: skipping const checks
  --> /checkout/src/test/ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv.rs:18:1
   |
LL | / const fn calling_attributed() -> L { //~ WARN skipping const checks
LL | |     // We need `-Z unleash-the-miri-inside-of-you` for this as we don't have `const fn` pointers.
LL | |     let ptr: fn() -> L = attributed;
LL | |     ptr()
   | |_^

warning: 1 warning emitted


error: internal compiler error: skipping const checks is for testing error paths only
  --> /checkout/src/test/ui/rfc-2091-track-caller/caller-location-fnptr-rt-ctfe-equiv.rs:21:5
   |
LL |     ptr()
   |     ^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (82784f4e9 2020-05-01) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0

------------------------------------------


---
test result: FAILED. 9892 passed; 9 failed; 62 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:55:12
Build completed unsuccessfully in 0:55:12
== clock drift check ==
  local time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Fri May  1 12:18:39 UTC 2020
  network time: Fri, 01 May 2020 12:18:40 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71631/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71631/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3628) (python)
##[section]Finishing: Finalize Job

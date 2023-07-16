plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/073f0068-586b-4b26-9711-764668e66c20.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71824/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71824/merge:refs/remotes/pull/71824/merge
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
.................i.................................................................................. 1800/9971
.................................................................................................... 1900/9971
.................................i.................................................................. 2000/9971
.................................................................................................... 2100/9971
.......................iiiii........................................................................ 2200/9971
.................................................................................................... 2400/9971
.................................................................................................... 2500/9971
.................................................................................................... 2600/9971
.................................................................................................... 2700/9971
---
.......i...............i............................................................................ 5100/9971
.................................................................................................... 5200/9971
.....................................................i.............................................. 5300/9971
............................................i....................................................... 5400/9971
..............................................ii.ii........i...i.................................... 5500/9971
.............................................................................................i...... 5700/9971
.................................................................................................... 5800/9971
............................ii.....................................i................................ 5900/9971
.................................................................................................... 6000/9971
.................................................................................................... 6000/9971
.................................................................................................... 6100/9971
..............................................................ii...i..ii...........i................ 6200/9971
.................................................................................................... 6400/9971
.................................................................................................... 6500/9971
.................................................................................................... 6500/9971
..............................................................................................i..ii. 6600/9971
.................................................................................................... 6800/9971
...............................................................................................i.... 6900/9971
.................................................................................................... 7000/9971
.................................................................................................... 7100/9971
---
.................................................................................................... 7900/9971
.................................................................................................... 8000/9971
.................................................................................................... 8100/9971
.....i.............................................................................................. 8200/9971
...........................................................iiiiii.iiiii.i........................... 8300/9971
............i........................................F.............................................. 8500/9971
.................................................................................................... 8600/9971
..................F................................................................................. 8700/9971
.................................................................................................... 8800/9971
---

- error[E0493]: destructors cannot be evaluated at compile-time
-   --> $DIR/check-static-values-constraints.rs:65:43
-    |
- LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
- LL | |
- LL | |
- LL | |                                                      field2: SafeEnum::Variant1}};
-    | |________________________________________________________________________________^ statics cannot evaluate destructors
10 error[E0010]: allocations are not allowed in statics
11   --> $DIR/check-static-values-constraints.rs:79:33
12    |


106 LL |     let y = { static x: Box<isize> = box 3; x };
108 
- error: aborting due to 17 previous errors
+ error[E0493]: destructors cannot be evaluated at compile-time
+   --> $DIR/check-static-values-constraints.rs:61:81
+   --> $DIR/check-static-values-constraints.rs:61:81
+    |
+ LL | ...                   field2: SafeEnum::Variant1}};
+    |                                                  ^ statics cannot evaluate destructors
+ error[E0493]: destructors cannot be evaluated at compile-time
+   --> $DIR/check-static-values-constraints.rs:67:81
+    |
+    |
+ LL | ...                   field2: SafeEnum::Variant1}};
+    |                                                  ^ statics cannot evaluate destructors
+ error: aborting due to 18 previous errors
110 
111 Some errors have detailed explanations: E0010, E0015, E0019, E0493, E0507.
112 For more information about an error, try `rustc --explain E0010`.
112 For more information about an error, try `rustc --explain E0010`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/check-static-values-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;
   |                                 ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:37
   |
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;

error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/check-static-values-constraints.rs:90:32
   |
   |
LL |     field2: SafeEnum::Variant4("str".to_string())

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:9
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:97:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:97:9
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:102:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:102:10
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:104:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:104:10
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:111:5
   |
---

error[E0507]: cannot move out of static item `x`
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:45
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                             |
   |                                             |
   |                                             move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait
   |                                             help: consider borrowing here: `&x`
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:38
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                      ^^^^^ allocation not allowed in statics
error[E0019]: static contains unimplemented expression type
  --> /checkout/src/test/ui/check-static-values-constraints.rs:116:42
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/check-static-values-constraints.rs:61:81
   |
   |
LL | ...                   field2: SafeEnum::Variant1}};
   |                                                  ^ statics cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/check-static-values-constraints.rs:67:81
   |
   |
LL | ...                   field2: SafeEnum::Variant1}};
   |                                                  ^ statics cannot evaluate destructors
error: aborting due to 18 previous errors

Some errors have detailed explanations: E0010, E0015, E0019, E0493, E0507.
For more information about an error, try `rustc --explain E0010`.
---
1 error[E0493]: destructors cannot be evaluated at compile-time
-   --> $DIR/E0493.rs:17:17
+   --> $DIR/E0493.rs:17:48
3    |
4 LL | const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
+    |                                                ^ constants cannot evaluate destructors
6 
7 error: aborting due to previous error
8 
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/E0493.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/E0493.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/E0493.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/E0493/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/span/E0493.rs:17:48
   |
LL | const F : Foo = (Foo { a : 0 }, Foo { a : 1 }).1;
   |                                                ^ constants cannot evaluate destructors
error: aborting due to previous error

For more information about this error, try `rustc --explain E0493`.

---

- error[E0493]: destructors cannot be evaluated at compile-time
-   --> $DIR/static-drop-scope.rs:9:60
-    |
- LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                            ^^^^^^^^ statics cannot evaluate destructors
7 error[E0716]: temporary value dropped while borrowed
8   --> $DIR/static-drop-scope.rs:9:60
9    |


14    |                                                      |     creates a temporary which is freed while still in use
15    |                                                      using this value as a static requires that borrow lasts for `'static`
16 
- error[E0493]: destructors cannot be evaluated at compile-time
-   --> $DIR/static-drop-scope.rs:13:59
-    |
- LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
- 
23 error[E0716]: temporary value dropped while borrowed
24   --> $DIR/static-drop-scope.rs:13:59
25    |
---
+ 
+ error[E0493]: destructors cannot be evaluated at compile-time
+   --> $DIR/static-drop-scope.rs:9:60
+    |
+ LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
+    |                                                            ^^^^^^^^ statics cannot evaluate destructors
+ error[E0493]: destructors cannot be evaluated at compile-time
+   --> $DIR/static-drop-scope.rs:13:59
+    |
+    |
+ LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
32 
33 error[E0493]: destructors cannot be evaluated at compile-time
34   --> $DIR/static-drop-scope.rs:17:28



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-drop-scope.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                      |     |       |
   |                                                      |     |       temporary value is freed at the end of this statement
   |                                                      |     creates a temporary which is freed while still in use
   |                                                      using this value as a static requires that borrow lasts for `'static`
   |                                                      using this value as a static requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                     |     |       |
   |                                                     |     |       temporary value is freed at the end of this statement
   |                                                     |     creates a temporary which is freed while still in use
   |                                                     using this value as a constant requires that borrow lasts for `'static`
   |                                                     using this value as a constant requires that borrow lasts for `'static`

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:9:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                            ^^^^^^^^ statics cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:13:59
   |
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:17:28
   |
   |
LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
   |                            ^^^^^^^^^^^^^ statics cannot evaluate destructors
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:20:27
   |
   |
LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:23:24
   |
   |
LL | const fn const_drop<T>(_: T) {}
   |                        ^ constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:27:5
   |
LL |     (x, ()).1

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:31:34
   |
   |
LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:36:43
   |
   |
LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0493, E0716.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:58:14
Build completed unsuccessfully in 0:58:14
== clock drift check ==
  local time: Sun May  3 05:26:39 UTC 2020
  network time: Sun, 03 May 2020 05:26:39 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71824/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71824/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4437) (python)
##[section]Finishing: Finalize Job

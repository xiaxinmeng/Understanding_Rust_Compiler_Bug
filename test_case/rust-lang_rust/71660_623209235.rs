plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/914eb238-35f4-40ea-8b4b-6767b286e014.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71660/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71660/merge:refs/remotes/pull/71660/merge
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
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
.................i.................................................................................. 1800/9979
.................................................................................................... 1900/9979
.................................i.................................................................. 2000/9979
.................................................................................................... 2100/9979
.......................iiiii........................................................................ 2200/9979
.................................................................................................... 2400/9979
.................................................................................................... 2500/9979
.................................................................................................... 2600/9979
.................................................................................................... 2700/9979
---
...........i...............i........................................................................ 5100/9979
.................................................................................................... 5200/9979
.........................................................i.......................................... 5300/9979
................................................i................................................... 5400/9979
....................................................ii.ii........i...i.............................. 5500/9979
...................................................................................................i 5700/9979
.................................................................................................... 5800/9979
...................................ii.....................................i......................... 5900/9979
.................................................................................................... 6000/9979
.................................................................................................... 6000/9979
.................................................................................................... 6100/9979
......................................................................ii...i..ii...........i........ 6200/9979
.................................................................................................... 6400/9979
.................................................................................................... 6500/9979
.................................................................................................... 6600/9979
.................................................................................................... 6600/9979
..i..ii............................................................................................. 6700/9979
.................................................................................................... 6900/9979
...i................................................................................................ 7000/9979
.................................................................................................... 7100/9979
.............................................i...................................................... 7200/9979
---
.................................................................................................... 7900/9979
.................................................................................................... 8000/9979
.................................................................................................... 8100/9979
.............i...................................................................................... 8200/9979
...................................................................iiiiii.iiiii.i................... 8300/9979
....................i............................................................................... 8500/9979
.................................................................................................... 8600/9979
.................................................................................................... 8700/9979
.................................................................................................... 8800/9979
---

---- [ui] ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs stdout ----
diff of stderr:

+ error[E0428]: the name `yes_ref_array_partial_eq_vec` is defined multiple times
+    |
+    |
+ LL | / pub fn yes_ref_array_partial_eq_vec<'a, A, B>(ref_array: &'a [A; 33]) -> impl PartialEq<Vec<B>> + 'a
+ LL | |
+ LL | | where
+ LL | |     A: PartialEq<B>,
+ LL | | {
+ LL | |     ref_array
+ LL | | }
+    | |_- previous definition of the value `yes_ref_array_partial_eq_vec` here
+ LL | 
+ LL | / pub fn yes_ref_array_partial_eq_vec<'a, A, B>(
+ LL | |     ref_mut_array: &'a mut [A; 33],
+ LL | | ) -> impl PartialEq<Vec<B>> + 'a
+ LL | |
+ ...  |
+ LL | |     ref_mut_array
+ LL | | }
+    | |_^ `yes_ref_array_partial_eq_vec` redefined here
+    |
+    = note: `yes_ref_array_partial_eq_vec` must be defined only once in the value namespace of this module
1 error[E0277]: arrays only have std trait implementations for lengths 0..=32
2   --> $DIR/alloc-traits-no-impls-length-33.rs:1:43
3    |


37 error[E0277]: arrays only have std trait implementations for lengths 0..=32
38   --> $DIR/alloc-traits-no-impls-length-33.rs:25:74
39    |
- LL | pub fn yes_ref_array_partial_eq_vec<'a, A, B>(ref_array: &'a [A; 33]) -> impl PartialEq<Vec<B>>
-    |                                                                          ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
+ LL | pub fn yes_ref_array_partial_eq_vec<'a, A, B>(ref_array: &'a [A; 33]) -> impl PartialEq<Vec<B>> + 'a
+    |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
- LL |    ref_array
- LL |    ref_array
-    |    --------- this returned value is of type `&'a [A; 33]`
+ LL |     ref_array
+    |     --------- this returned value is of type `&'a [A; 33]`
45    |
46    = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<B>>` for `&[A; 33]`

48 
49 error[E0277]: arrays only have std trait implementations for lengths 0..=32
-   --> $DIR/alloc-traits-no-impls-length-33.rs:35:48
-   --> $DIR/alloc-traits-no-impls-length-33.rs:35:48
+   --> $DIR/alloc-traits-no-impls-length-33.rs:35:6
51    |
+ LL | ) -> impl PartialEq<Vec<B>> + 'a
+    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
+ ...
+ LL |     ref_mut_array
+    |     ------------- this returned value is of type `&'a mut [A; 33]`
+    |
+    = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<B>>` for `&mut [A; 33]`
+ 
+ error[E0277]: arrays only have std trait implementations for lengths 0..=32
+   --> $DIR/alloc-traits-no-impls-length-33.rs:45:48
+    |
+    |
52 LL | pub fn no_vecdeque_partial_eq_array<A, B>() -> impl PartialEq<[B; 33]>
53    |                                                ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`

59    = note: the return type of a function must have a statically known size
60 
61 error[E0277]: arrays only have std trait implementations for lengths 0..=32
61 error[E0277]: arrays only have std trait implementations for lengths 0..=32
-   --> $DIR/alloc-traits-no-impls-length-33.rs:43:56
+   --> $DIR/alloc-traits-no-impls-length-33.rs:53:56
63    |
64 LL | pub fn no_vecdeque_partial_eq_ref_array<'a, A, B>() -> impl PartialEq<&'a [B; 33]>
65    |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
71    = note: the return type of a function must have a statically known size
72 
73 error[E0277]: arrays only have std trait implementations for lengths 0..=32
-   --> $DIR/alloc-traits-no-impls-length-33.rs:51:60
-   --> $DIR/alloc-traits-no-impls-length-33.rs:51:60
+   --> $DIR/alloc-traits-no-impls-length-33.rs:61:60
75    |
76 LL | pub fn no_vecdeque_partial_eq_ref_mut_array<'a, A, B>() -> impl PartialEq<&'a mut [B; 33]>
77    |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`

82    = note: required because of the requirements on the impl of `std::cmp::PartialEq<&'a mut [B; 33]>` for `std::collections::VecDeque<A>`
84 
- error: aborting due to 7 previous errors
+ error: aborting due to 9 previous errors
86 
86 
- For more information about this error, try `rustc --explain E0277`.
+ Some errors have detailed explanations: E0277, E0428.
+ For more information about an error, try `rustc --explain E0277`.
88 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33/alloc-traits-no-impls-length-33.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/array-impls/alloc-traits-no-impls-length-33.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0428]: the name `yes_ref_array_partial_eq_vec` is defined multiple times
   |
   |
LL | / pub fn yes_ref_array_partial_eq_vec<'a, A, B>(ref_array: &'a [A; 33]) -> impl PartialEq<Vec<B>> + 'a
LL | | //~^ ERROR arrays only have std trait implementations for lengths 0..=32
LL | | where
LL | |     A: PartialEq<B>,
LL | | {
LL | |     ref_array
LL | | }
   | |_- previous definition of the value `yes_ref_array_partial_eq_vec` here
LL | 
LL | / pub fn yes_ref_array_partial_eq_vec<'a, A, B>(
LL | |     ref_mut_array: &'a mut [A; 33],
LL | | ) -> impl PartialEq<Vec<B>> + 'a
LL | | //~^ ERROR arrays only have std trait implementations for lengths 0..=32
LL | |     ref_mut_array
LL | | }
LL | | }
   | |_^ `yes_ref_array_partial_eq_vec` redefined here
   |
   = note: `yes_ref_array_partial_eq_vec` must be defined only once in the value namespace of this module
error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:1:43
   |
   |
LL | pub fn no_vec_partial_eq_array<A, B>() -> impl PartialEq<[B; 33]>
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
...
LL |     Vec::<A>::new()
   |     --------------- this returned value is of type `std::vec::Vec<A>`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<[B; 33]>` for `std::vec::Vec<A>`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:9:51
   |
   |
LL | pub fn no_vec_partial_eq_ref_array<'a, A, B>() -> impl PartialEq<&'a [B; 33]>
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
...
LL |     Vec::<A>::new()
   |     --------------- this returned value is of type `std::vec::Vec<A>`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<&'a [B; 33]>` for `std::vec::Vec<A>`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:17:58
   |
   |
LL | pub fn yes_array_partial_eq_vec<A, B>(array: [A; 33]) -> impl PartialEq<Vec<B>>
   |                                                          ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
LL |     array
LL |     array
   |     ----- this returned value is of type `[A; 33]`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<B>>` for `[A; 33]`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:25:74
   |
   |
LL | pub fn yes_ref_array_partial_eq_vec<'a, A, B>(ref_array: &'a [A; 33]) -> impl PartialEq<Vec<B>> + 'a
   |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
LL |     ref_array
LL |     ref_array
   |     --------- this returned value is of type `&'a [A; 33]`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<B>>` for `&[A; 33]`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:35:6
   |
   |
LL | ) -> impl PartialEq<Vec<B>> + 'a
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[A; 33]`
LL |     ref_mut_array
LL |     ref_mut_array
   |     ------------- this returned value is of type `&'a mut [A; 33]`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<std::vec::Vec<B>>` for `&mut [A; 33]`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:45:48
   |
   |
LL | pub fn no_vecdeque_partial_eq_array<A, B>() -> impl PartialEq<[B; 33]>
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
...
LL |     VecDeque::<A>::new()
   |     -------------------- this returned value is of type `std::collections::VecDeque<A>`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<[B; 33]>` for `std::collections::VecDeque<A>`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:53:56
   |
   |
LL | pub fn no_vecdeque_partial_eq_ref_array<'a, A, B>() -> impl PartialEq<&'a [B; 33]>
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
...
LL |     VecDeque::<A>::new()
   |     -------------------- this returned value is of type `std::collections::VecDeque<A>`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<&'a [B; 33]>` for `std::collections::VecDeque<A>`

error[E0277]: arrays only have std trait implementations for lengths 0..=32
  --> /checkout/src/test/ui/const-generics/array-impls/alloc-traits-no-impls-length-33.rs:61:60
   |
   |
LL | pub fn no_vecdeque_partial_eq_ref_mut_array<'a, A, B>() -> impl PartialEq<&'a mut [B; 33]>
   |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[B; 33]`
...
LL |     VecDeque::<A>::new()
   |     -------------------- this returned value is of type `std::collections::VecDeque<A>`
   |
   = note: required because of the requirements on the impl of `std::cmp::PartialEq<&'a mut [B; 33]>` for `std::collections::VecDeque<A>`

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0277, E0428.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:11:45
Build completed unsuccessfully in 1:11:45
== clock drift check ==
  local time: Mon May  4 00:24:25 UTC 2020
  network time: Mon, 04 May 2020 00:24:25 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71660/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71660/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3605) (python)
##[section]Finishing: Finalize Job

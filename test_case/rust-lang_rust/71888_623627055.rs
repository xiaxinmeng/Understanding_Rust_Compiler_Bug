plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/df022d56-3df2-43b1-af21-67879d61bd36.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71888/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71888/merge:refs/remotes/pull/71888/merge
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
..................i................................................................................. 1800/9983
.................................................................................................... 1900/9983
...................................i................................................................ 2000/9983
.................................................................................................... 2100/9983
.........................iiiii...................................................................... 2200/9983
.................................................................................................... 2400/9983
.................................................................................................... 2500/9983
.................................................................................................... 2600/9983
.................................................................................................... 2700/9983
---
.............i...............i...................................................................... 5100/9983
.................................................................................................... 5200/9983
...........................................................i........................................ 5300/9983
..................................................i................................................. 5400/9983
......................................................ii.ii........i...i............................ 5500/9983
.................................................................................................... 5700/9983
.i.................................................................................................. 5800/9983
.....................................ii.....................................i....................... 5900/9983
.................................................................................................... 6000/9983
.................................................................................................... 6000/9983
.................................................................................................... 6100/9983
........................................................................ii...i..ii...........i...... 6200/9983
.................................................................................................... 6400/9983
.................................................................................................... 6500/9983
.................................................................................................... 6600/9983
.................................................................................................... 6600/9983
....i..ii........................................................................................... 6700/9983
.................................................................................................... 6900/9983
.....i.............................................................................................. 7000/9983
.................................................................................................... 7100/9983
...............................................i.................................................... 7200/9983
---
.................................................................................................... 7900/9983
.................................................................................................... 8000/9983
.................................................................................................... 8100/9983
...............i.................................................................................... 8200/9983
.....................................................................iiiiii.iiiii.i................. 8300/9983
......................i............................................................................. 8500/9983
.................................................................................................... 8600/9983
.................................................................................................... 8700/9983
.................................................................................................... 8800/9983
---
---- [ui] ui/associated-const/associated-const-no-item.rs stdout ----
diff of stderr:

3    |
4 LL | const X: i32 = <i32>::ID;
5    |                       ^^ associated item not found in `i32`
-    = help: items from traits can only be used if the trait is implemented and in scope
-    = help: items from traits can only be used if the trait is implemented and in scope
- note: `Foo` defines an item `ID`, perhaps you need to implement it
-    |
- LL | trait Foo {
-    | ^^^^^^^^^
13 
13 
14 error: aborting due to previous error
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item/associated-const-no-item.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-const/associated-const-no-item.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/associated-const-no-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/associated-const-no-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no associated item named `ID` found for type `i32` in the current scope
  --> /checkout/src/test/ui/associated-const/associated-const-no-item.rs:5:23
   |
LL | const X: i32 = <i32>::ID;
   |                       ^^ associated item not found in `i32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui/associated-item/associated-item-duplicate-bounds.rs stdout ----
diff of stderr:

3    |
4 LL |     links: [u32; A::LINKS], // Shouldn't suggest bounds already there.
5    |                     ^^^^^ associated item not found in `A`
-    = help: items from traits can only be used if the type parameter is bounded by the trait
8 
9 error: aborting due to previous error
10 
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-duplicate-bounds/associated-item-duplicate-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-item/associated-item-duplicate-bounds.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-item/associated-item-duplicate-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-duplicate-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-duplicate-bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no associated item named `LINKS` found for type parameter `A` in the current scope
  --> /checkout/src/test/ui/associated-item/associated-item-duplicate-bounds.rs:7:21
   |
LL |     links: [u32; A::LINKS], // Shouldn't suggest bounds already there.
   |                     ^^^^^ associated item not found in `A`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.

---
88 LL |     S::A;
89    |        ^ associated item not found in `S`
-    |
-    = help: items from traits can only be used if the trait is implemented and in scope
- note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
-    |
- LL |     trait A {
-    |     ^^^^^^^
97 
97 
98 error[E0599]: no associated item named `B` found for struct `S` in the current scope
99   --> $DIR/trait-item-privacy.rs:98:8


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/trait-item-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/trait-item-privacy.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for struct `S` in the current scope
   |
LL | struct S;
   | --------- method `a` not found for this
...
...
LL |     S.a(); //~ ERROR no method named `a` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no method named `b` found for struct `S` in the current scope
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:68:7
   |
LL | struct S;
   | --------- method `b` not found for this
...
LL |         fn b(&self) { }
   |            |
   |            |
   |            the method is available for `std::boxed::Box<S>` here
   |            the method is available for `std::sync::Arc<S>` here
   |            the method is available for `std::rc::Rc<S>` here
...
LL |     S.b(); //~ ERROR no method named `b` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:72:7
   |
LL |     c.a(); //~ ERROR associated function `a` is private
   |       ^ private associated function

error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
   |
LL | struct S;
   | --------- function or associated item `a` not found for this
...
...
LL |     S::a(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:80:8
   |
LL | struct S;
   | --------- function or associated item `b` not found for this
...
LL |     S::b(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:84:8
   |
LL |     C::a(&S); //~ ERROR associated function `a` is private
   |        ^ private associated function

error[E0599]: no associated item named `A` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `A` not found for this
...
LL |     S::A; //~ ERROR no associated item named `A` found
   |        ^ associated item not found in `S`
error[E0599]: no associated item named `B` found for struct `S` in the current scope
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:98:8
   |
LL | struct S;
LL | struct S;
   | --------- associated item `B` not found for this
...
LL |     S::B; //~ ERROR no associated item named `B` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use assoc_const::B;
LL | use assoc_const::B;
   |

error[E0624]: associated constant `A` is private
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:8
   |
LL |     C::A; //~ ERROR associated constant `A` is private
   |        ^ private associated constant
error[E0038]: the trait `assoc_const::C` cannot be made into an object
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:101:5
   |
LL |         const A: u8 = 0;
LL |         const A: u8 = 0;
   |               - ...because it contains this associated `const`
...
LL |         const B: u8 = 0;
   |               - ...because it contains this associated `const`
...
LL |     pub trait C: A + B {
LL |         const C: u8 = 0;
   |               - ...because it contains this associated `const`
...
...
LL |     C::A; //~ ERROR associated constant `A` is private
   |     ^^^^ the trait `assoc_const::C` cannot be made into an object
   = help: consider moving `C` to another trait
   = help: consider moving `B` to another trait
   = help: consider moving `A` to another trait


error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:115:12
   |
LL |     let _: S::A; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:116:12
   |
   |
LL |     let _: S::B; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:117:12
   |
   |
LL |     let _: S::C; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`
error: associated type `A` is private
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:119:12
   |
   |
LL |     let _: T::A; //~ ERROR associated type `A` is private

error: associated type `A` is private
  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:128:9
   |
   |
LL |         A = u8, //~ ERROR associated type `A` is private

error: aborting due to 15 previous errors

Some errors have detailed explanations: E0038, E0223, E0599, E0624.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:04:23
Build completed unsuccessfully in 1:04:23
== clock drift check ==
  local time: Mon May  4 18:23:11 UTC 2020
  network time: Mon, 04 May 2020 18:23:11 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71888/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71888/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3683) (python)
##[section]Finishing: Finalize Job

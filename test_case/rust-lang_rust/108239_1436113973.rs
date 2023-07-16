plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
To only update this specific test, also pass `--test-args alloc-error/alloc-error-handler-bad-signature-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/alloc-error/alloc-error-handler-bad-signature-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alloc-error/alloc-error-handler-bad-signature-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alloc-error/alloc-error-handler-bad-signature-3/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/alloc-error/alloc-error-handler-bad-signature-3.rs:10:1
   |
LL |   #[alloc_error_handler]
   |   ---------------------- in this procedural macro expansion
   |   ---------------------- in this procedural macro expansion
LL |   fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
   |  _-^^^^^^^^^^^^
LL | |     loop {}
LL | | }
   | |_- unexpected argument of type `core::alloc::Layout`
note: function defined here
  --> fake-test-src-base/alloc-error/alloc-error-handler-bad-signature-3.rs:10:4
   |
   |
LL | fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
   = note: this error originates in the attribute macro `alloc_error_handler` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
diff of stderr:

2   --> $DIR/issue-26094.rs:10:17
3    |
4 LL |         $other(None)
-    |                |
-    |                unexpected argument of type `Option<_>`
-    |                help: remove the extra argument
+    |                ---- unexpected argument of type `Option<_>`
+    |                ---- unexpected argument of type `Option<_>`
9 ...
10 LL |     some_macro!(some_function);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26094/issue-26094.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26094/issue-26094.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-26094.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-26094.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26094" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26094/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/issues/issue-26094.rs:10:17
   |
   |
LL |         $other(None) //~ NOTE unexpected argument of type `Option<_>`
   |                ---- unexpected argument of type `Option<_>`
...
LL |     some_macro!(some_function);
   |
note: function defined here
  --> fake-test-src-base/issues/issue-26094.rs:7:4
   |
   |
LL | fn some_function() {} //~ NOTE defined here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
---
diff of stderr:

14   --> $DIR/struct-enum-wrong-args.rs:7:13
15    |
16 LL |     let _ = Ok(3, 6, 2);
-    |             ^^    -  - unexpected argument of type `{integer}`
-    |                   unexpected argument of type `{integer}`
+    |             ^^  ------
+    |                 | |
+    |                 | 2 unexpected arguments
+    |                 | 2 unexpected arguments
+    |                 help: remove the extra arguments
20    |
21 note: tuple variant defined here
22   --> $SRC_DIR/core/src/result.rs:LL:COL

- help: remove the extra arguments
-    |
- LL -     let _ = Ok(3, 6, 2);
- LL +     let _ = Ok(3);
28 
29 error[E0061]: this enum variant takes 1 argument but 0 arguments were supplied
30   --> $DIR/struct-enum-wrong-args.rs:8:13

---
To only update this specific test, also pass `--test-args typeck/struct-enum-wrong-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/struct-enum-wrong-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:6:13
   |
   |
LL |     let _ = Some(3, 2); //~ ERROR this enum variant takes
   |             ^^^^  ---
   |                   | unexpected argument of type `{integer}`
   |                   help: remove the extra argument
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
note: tuple variant defined here
  --> /rustc/FAKE_PREFIX/library/core/src/option.rs:570:5

error[E0061]: this enum variant takes 1 argument but 3 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:7:13
   |
LL |     let _ = Ok(3, 6, 2); //~ ERROR this enum variant takes
   |             ^^  ------
   |                 | 2 unexpected arguments
   |                 help: remove the extra arguments
   |
note: tuple variant defined here
note: tuple variant defined here
  --> /rustc/FAKE_PREFIX/library/core/src/result.rs:507:5

error[E0061]: this enum variant takes 1 argument but 0 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:8:13
   |
LL |     let _ = Ok(); //~ ERROR this enum variant takes
   |             ^^-- an argument is missing
note: tuple variant defined here
  --> /rustc/FAKE_PREFIX/library/core/src/result.rs:507:5
help: provide the argument
   |
   |
LL |     let _ = Ok(/* value */); //~ ERROR this enum variant takes

error[E0061]: this struct takes 1 argument but 0 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:9:13
   |
   |
LL |     let _ = Wrapper(); //~ ERROR this struct takes
   |             ^^^^^^^-- an argument of type `i32` is missing
note: tuple struct defined here
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:2:8
   |
   |
LL | struct Wrapper(i32);
help: provide the argument
   |
   |
LL |     let _ = Wrapper(/* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 1 argument but 2 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:10:13
   |
   |
LL |     let _ = Wrapper(5, 2); //~ ERROR this struct takes
   |             ^^^^^^^  ---
   |                      | unexpected argument of type `{integer}`
   |                      help: remove the extra argument
   |
note: tuple struct defined here
note: tuple struct defined here
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:2:8
   |
LL | struct Wrapper(i32);

error[E0061]: this struct takes 2 arguments but 0 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:11:13
   |
   |
LL |     let _ = DoubleWrapper(); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^-- two arguments of type `i32` and `i32` are missing
note: tuple struct defined here
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:3:8
   |
   |
LL | struct DoubleWrapper(i32, i32);
help: provide the arguments
   |
   |
LL |     let _ = DoubleWrapper(/* i32 */, /* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 2 arguments but 1 argument was supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:12:13
   |
   |
LL |     let _ = DoubleWrapper(5); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^--- an argument of type `i32` is missing
note: tuple struct defined here
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:3:8
   |
   |
LL | struct DoubleWrapper(i32, i32);
help: provide the argument
   |
   |
LL |     let _ = DoubleWrapper(5, /* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 2 arguments but 3 arguments were supplied
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:13:13
   |
   |
LL |     let _ = DoubleWrapper(5, 2, 7); //~ ERROR this struct takes
   |                               | |
   |                               | unexpected argument of type `{integer}`
   |                               help: remove the extra argument
   |
   |
note: tuple struct defined here
  --> fake-test-src-base/typeck/struct-enum-wrong-args.rs:3:8
   |
LL | struct DoubleWrapper(i32, i32);

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0061`.

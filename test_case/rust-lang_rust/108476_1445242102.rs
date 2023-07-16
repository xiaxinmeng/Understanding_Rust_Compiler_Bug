plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/consts/min_const_fn/bad_const_fn_body_ice.rs stdout ----
diff of stderr:

- error[E0010]: allocations are not allowed in constant functions
+ error[E0015]: cannot call non-const fn `Box::<[i32; 3]>::new` in constant functions
3    |
4 LL |     vec![1, 2, 3]

-    |     ^^^^^^^^^^^^^ allocation not allowed in constant functions
-    |     ^^^^^^^^^^^^^ allocation not allowed in constant functions
+    |     ^^^^^^^^^^^^^
6    |
+    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
7    = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
9 error[E0015]: cannot call non-const fn `slice::<impl [i32]>::into_vec::<std::alloc::Global>` in constant functions
17 
18 error: aborting due to 2 previous errors
19 
- Some errors have detailed explanations: E0010, E0015.
---
To only update this specific test, also pass `--test-args consts/min_const_fn/bad_const_fn_body_ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/min_const_fn/bad_const_fn_body_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/bad_const_fn_body_ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/bad_const_fn_body_ice/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const fn `Box::<[i32; 3]>::new` in constant functions
  --> fake-test-src-base/consts/min_const_fn/bad_const_fn_body_ice.rs:2:5
LL |     vec![1, 2, 3]
   |     ^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0015]: cannot call non-const fn `slice::<impl [i32]>::into_vec::<std::alloc::Global>` in constant functions
  --> fake-test-src-base/consts/min_const_fn/bad_const_fn_body_ice.rs:2:5
LL |     vec![1, 2, 3]
   |     ^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/type/ascription/issue-47666.rs stdout ----
diff of stderr:

- error: expected type, found `<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0, 1]))`
+ error: expected type, found `<[_]>::into_vec(::alloc::boxed::Box::new([0, 1]))`
3    |
3    |
4 LL |     let _ = Option:Some(vec![0, 1]);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/issue-47666.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/ascription/issue-47666.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/ascription/issue-47666.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-47666/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected type, found `<[_]>::into_vec(::alloc::boxed::Box::new([0, 1]))`
  --> fake-test-src-base/type/ascription/issue-47666.rs:3:25
   |
LL |     let _ = Option:Some(vec![0, 1]); //~ ERROR expected type, found
   |                   |     |
   |                   |     expected type
   |                   |     in this macro invocation
   |                   |     this macro call doesn't expand to a type
   |                   |     this macro call doesn't expand to a type
   |                   help: maybe write a path separator here: `::`
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
   = note: this error originates in the macro `$crate::__rust_force_expr` which comes from the expansion of the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error
------------------------------------------



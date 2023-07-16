plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
diff of 32bit.stderr:

15 LL | const FOO: [empty::Empty; 3] = [foo(); 3];
17 
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_uninhabited_zsts.rs:21:1
+ note: erroneous constant used
---
+ 
+ error[E0080]: evaluation of constant value failed
+   --> $DIR/validate_uninhabited_zsts.rs:21:42
20    |
21 LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
22    |                                          ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered a value of uninhabited type empty::Void

The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
---
   |              ^^^^^^^^^^^^^^^^^^^^^^^
note: inside `FOO`
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:19:33
   |
LL | const FOO: [empty::Empty; 3] = [foo(); 3];

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:26:5
   |
---

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:21:42
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered a value of uninhabited type empty::Void
note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:27:5
   |
LL |     BAR;
---
   |
LL |     BAR;
   |     ^^^

warning: the type `!` does not permit zero-initialization
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:4:14
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^ this code causes undefined behavior when executed
   |
   |
   = note: the `!` type has no valid value


warning: the type `empty::Empty` does not permit zero-initialization
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:21:42
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |
note: in this struct field
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:16:22
   |
   |
LL |     pub struct Empty(Void);
   |                      ^^^^
note: enums with no inhabited variants have no valid value
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:13:5
LL |     enum Void {}
   |     ^^^^^^^^^

error: aborting due to 2 previous errors; 2 warnings emitted

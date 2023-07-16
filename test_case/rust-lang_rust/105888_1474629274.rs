plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [ui] tests/ui/lifetimes/issue-79187-2.rs stdout ----
diff of stderr:

43    |
44 LL | fn take_foo(_: impl Foo) {}
- help: consider changing the type of the closure parameters
+ help: consider specifying the type of the closure parameters
47    |
47    |
- LL |     take_foo(|_: &_| a);
+ LL |     take_foo(|a: &_| a);
50 
51 error[E0308]: mismatched types



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/issue-79187-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-79187-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:11:24
   |
LL |     take_foo(|a: &i32| a);
   |                  -   - ^ returning this value requires that `'1` must outlive `'2`
   |                  |   |
   |                  |   return type of closure is &'2 i32
   |                  let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:14:34
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |                  -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                  |        |
   |                  |        let's call the lifetime of this reference `'2`
   |                  let's call the lifetime of this reference `'1`

error: implementation of `FnOnce` is not general enough
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:8:5
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 i32) -> &i32` must implement `FnOnce<(&'1 i32,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 i32,)>`, for some specific lifetime `'2`
error[E0308]: mismatched types
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:8:5
   |
   |
LL |     take_foo(|a| a);
   |     ^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'a> Fn<(&'a i32,)>`
              found trait `Fn<(&i32,)>`
note: this closure does not fulfill the lifetime requirements
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:8:14
   |
LL |     take_foo(|a| a);
note: the lifetime requirement is introduced here
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}
help: consider specifying the type of the closure parameters
   |
   |
LL |     take_foo(|a: &_| a);

error[E0308]: mismatched types
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:11:5
   |
   |
LL |     take_foo(|a: &i32| a);
   |     ^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:5:21
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:14:5
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a });
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&i32`
              found reference `&i32`
note: the lifetime requirement is introduced here
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:5:21
  --> fake-test-src-base/lifetimes/issue-79187-2.rs:5:21
   |
LL | fn take_foo(_: impl Foo) {}

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0308`.

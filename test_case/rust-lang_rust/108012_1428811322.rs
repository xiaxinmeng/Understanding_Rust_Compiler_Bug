plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 14432/14447
...............
failures:

---- [ui] tests/ui/lint/invalid_value-polymorphic.rs stdout ----

- warning: use of deprecated function `std::mem::uninitialized`: use `mem::MaybeUninit` instead
-   --> $DIR/invalid_value-polymorphic.rs:9:43
-    |
-    |
- LL |         let _val: Wrap<&'static T> = mem::uninitialized();
-    |
-    = note: `#[warn(deprecated)]` on by default
- 
- 
- warning: the type `Wrap<&T>` does not permit being left uninitialized
-   --> $DIR/invalid_value-polymorphic.rs:9:38
-    |
- LL |         let _val: Wrap<&'static T> = mem::uninitialized();
-    |                                      |
-    |                                      this code causes undefined behavior when executed
-    |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
-    |
-    |
-    = note: `Wrap<&T>` must be non-null
- note: because references must be non-null (in this struct field)
-   --> $DIR/invalid_value-polymorphic.rs:5:18
-    |
- LL | struct Wrap<T> { wrapped: T }
-    = note: `#[warn(invalid_value)]` on by default
- 
- warning: 2 warnings emitted
- 
---
To only update this specific test, also pass `--test-args lint/invalid_value-polymorphic.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/lint/invalid_value-polymorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/invalid_value-polymorphic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/invalid_value-polymorphic/auxiliary" "--crate-type=lib" "-Zmir-enable-passes=+InstCombine" "-Zinline-mir"
stdout: none
stderr: none


failures:
    [ui] tests/ui/lint/invalid_value-polymorphic.rs

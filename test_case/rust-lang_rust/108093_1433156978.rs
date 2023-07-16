plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/traits/alias/cross-crate.rs stdout ----
diff of stderr:

5    |                 ^^^^^^^ `Rc<u32>` cannot be sent between threads safely
6    |
7    = help: the trait `Send` is not implemented for `Rc<u32>`
-    = note: required for `Rc<u32>` to implement `SendSync`
9 note: required by a bound in `use_alias`
11    |


19    |                 ^^^^^^^ `Rc<u32>` cannot be shared between threads safely
20    |
21    = help: the trait `Sync` is not implemented for `Rc<u32>`
-    = note: required for `Rc<u32>` to implement `SendSync`
23 note: required by a bound in `use_alias`
25    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate/cross-crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/cross-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/alias/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `Rc<u32>` cannot be sent between threads safely
  --> fake-test-src-base/traits/alias/cross-crate.rs:14:17
   |
LL |     use_alias::<Rc<u32>>();
   |                 ^^^^^^^ `Rc<u32>` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `Rc<u32>`
note: required by a bound in `use_alias`
  --> fake-test-src-base/traits/alias/cross-crate.rs:10:17
   |
LL | fn use_alias<T: SendSync>() {}
   |                 ^^^^^^^^ required by this bound in `use_alias`

error[E0277]: `Rc<u32>` cannot be shared between threads safely
  --> fake-test-src-base/traits/alias/cross-crate.rs:14:17
   |
LL |     use_alias::<Rc<u32>>();
   |                 ^^^^^^^ `Rc<u32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `Rc<u32>`
note: required by a bound in `use_alias`
  --> fake-test-src-base/traits/alias/cross-crate.rs:10:17
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | fn use_alias<T: SendSync>() {}
   |                 ^^^^^^^^ required by this bound in `use_alias`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------

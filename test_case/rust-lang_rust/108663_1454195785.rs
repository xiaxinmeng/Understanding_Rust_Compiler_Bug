plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
normalized stderr:
warning: where clauses are not enforced in type aliases
  --> $DIR/closure_parent_substs.rs:31:9
   |
LL |         &'a (): Trait,
   |
   |
   = note: `#[warn(type_alias_bounds)]` on by default
help: the clause will not be checked when the type alias is used, and should be removed
LL -     where
LL -     where
LL -         &'a (): Trait,
LL +     

warning: where clauses are not enforced in type aliases
  --> $DIR/closure_parent_substs.rs:52:9
   |
   |
LL |         (&'a (), &'b ()): Trait,
   |
   |
help: the clause will not be checked when the type alias is used, and should be removed
LL -     where
LL -     where
LL -         (&'a (), &'b ()): Trait,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL +     

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args type-alias-impl-trait/closure_parent_substs.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/closure_parent_substs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/closure_parent_substs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/closure_parent_substs/auxiliary"
stdout: none
--- stderr -------------------------------
warning: where clauses are not enforced in type aliases
  --> fake-test-src-base/type-alias-impl-trait/closure_parent_substs.rs:31:9
   |
LL |         &'a (): Trait,
   |
   |
   = note: `#[warn(type_alias_bounds)]` on by default
help: the clause will not be checked when the type alias is used, and should be removed
LL -     where
LL -     where
LL -         &'a (): Trait,
LL +     

warning: where clauses are not enforced in type aliases
  --> fake-test-src-base/type-alias-impl-trait/closure_parent_substs.rs:52:9
   |
   |
LL |         (&'a (), &'b ()): Trait,
   |
   |
help: the clause will not be checked when the type alias is used, and should be removed
LL -     where
LL -     where
LL -         (&'a (), &'b ()): Trait,
LL +     

warning: 2 warnings emitted
------------------------------------------


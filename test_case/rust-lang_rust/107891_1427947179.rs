plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7b992ce41a6506cf6388fd9a3d24a6eaebaa1d31)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 5104/14445
........................................................................................ 5192/14445
........................................................................................ 5280/14445
........................................................................................ 5368/14445
.................F....F................................................................. 5456/14445
........................................................................................ 5632/14445
..................................................i..........iiii....................... 5720/14445
.................................................i...................................... 5808/14445
........................................................................................ 5896/14445
---
---- [ui] tests/ui/impl-trait/nested-return-type2-tait2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/nested-return-type2-tait2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: opaque type `Traitable` does not satisfy its associated type bounds
  --> fake-test-src-base/impl-trait/nested-return-type2-tait2.rs:19:29
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `Sendable`
...
LL | type Traitable = impl Trait<Assoc = Sendable>;
   |
   = note: `#[warn(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Sendable = impl Send + Duh;

warning: 1 warning emitted
------------------------------------------



---- [ui] tests/ui/impl-trait/nested-return-type2-tait3.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/nested-return-type2-tait3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait3/auxiliary"
stdout: none
--- stderr -------------------------------
warning: opaque type `Traitable` does not satisfy its associated type bounds
  --> fake-test-src-base/impl-trait/nested-return-type2-tait3.rs:18:29
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `impl Send`
...
LL | type Traitable = impl Trait<Assoc = impl Send>;
   |
   = note: `#[warn(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Traitable = impl Trait<Assoc = impl Send + Duh>;

warning: 1 warning emitted
------------------------------------------


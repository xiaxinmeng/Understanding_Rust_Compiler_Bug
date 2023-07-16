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

---- [ui] tests/rustdoc-ui/unable-fulfill-trait.rs stdout ----
diff of stderr:

- error[E0107]: this trait takes 1 generic argument but 0 generic arguments were supplied
+ error[E0107]: trait takes 1 generic argument but 0 generic arguments were supplied
3    |
3    |
4 LL |     field1: dyn Bar<'a, 'b,>,

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unable-fulfill-trait/unable-fulfill-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unable-fulfill-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/unable-fulfill-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unable-fulfill-trait" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unable-fulfill-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: trait takes 1 generic argument but 0 generic arguments were supplied
   |
   |
LL |     field1: dyn Bar<'a, 'b,>,
   |                 ^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `U`
   |
   |
LL | pub trait Bar<'x, 's, U>
help: add missing generic argument
   |
   |
LL |     field1: dyn Bar<'a, 'b, U,>,


error[E0227]: ambiguous lifetime bound, explicit lifetime bound required
   |
   |
LL |     field1: dyn Bar<'a, 'b,>,

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0107, E0227.

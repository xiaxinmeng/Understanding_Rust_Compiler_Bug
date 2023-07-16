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
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........................
failures:

---- [ui] tests/rustdoc-ui/proc_macro_bug.rs stdout ----


4 LL | #[proc_macro_derive(DeriveA)]
6 
- error: Compilation failed, aborting rustdoc
- 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
10 
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/proc_macro_bug/proc_macro_bug.stderr
To only update this specific test, also pass `--test-args proc_macro_bug.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/proc_macro_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/proc_macro_bug" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/proc_macro_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
  --> /checkout/tests/rustdoc-ui/proc_macro_bug.rs:8:1
   |
LL | #[proc_macro_derive(DeriveA)]

error: aborting due to previous error
------------------------------------------


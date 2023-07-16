plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/consts/const_in_pattern/issue-73431.rs stdout ----
diff of stderr:

- WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
+ 2:rustcWARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_in_pattern/issue-73431.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_in_pattern/issue-73431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/auxiliary"
stdout: none
--- stderr -------------------------------
2:rustcWARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.


---- [ui] tests/ui/track-diagnostics/track6.rs stdout ----
diff of stderr:
diff of stderr:

3    |
4 LL |     default fn bar() {}
5    |     ^^^^^^^^^^^^^^^^^^^
- -Ztrack-diagnostics: created at $COMPILER_DIR/rustc_session/src/parse.rs:93:5
+ -Ztrack-diagnostics: created at $COMPILER_DIR/rustc_session/src/parse.rs:97:5
8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
8    = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
9    = help: add `#![feature(specialization)]` to the crate attributes to enable

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/track-diagnostics/track6/track6.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args track-diagnostics/track6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/track-diagnostics/track6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/track-diagnostics/track6" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/track-diagnostics/track6/auxiliary" "-Z" "track-diagnostics"
stdout: none
--- stderr -------------------------------
error[E0658]: specialization is unstable
  --> fake-test-src-base/track-diagnostics/track6.rs:11:5
LL |     default fn bar() {}
   |     ^^^^^^^^^^^^^^^^^^^
-Ztrack-diagnostics: created at /checkout/compiler/rustc_session/src/parse.rs:97:5
   |
   |
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: add `#![feature(specialization)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------

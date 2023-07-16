plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 67 tests
..........................F...FFFFFF.FF.FF...FFF...FF..............
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----


error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:16:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintPass};
   |                                             ^^^^^^^^^               ^^^^^^^^
warning: unused import: `rustc_ast::attr`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:18:5
   |
LL | use rustc_ast::attr;
LL | use rustc_ast::attr;
   |     ^^^^^^^^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:41:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:32:16
   |
LL |             cx.lint(CRATE_NOT_OKAY, |lint| {
   |                ^^^^ method not found in `&LateContext<'_>`
error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:67:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:34:28
   |
LL |                           cx.lint(CRATE_NOT_OKAY, |lint| {
   |                              ^^^^ method not found in `&LateContext<'_>`
...
LL | / fake_lint_pass! {
LL | |     PassOkay,
LL | |     Symbol::intern("crate_okay")
LL | | }
   |
   |
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:34:28
   |
LL |                           cx.lint(CRATE_NOT_OKAY, |lint| {
   |                              ^^^^ method not found in `&LateContext<'_>`
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
LL | | }
   |
   |
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:34:28
   |
LL |                           cx.lint(CRATE_NOT_OKAY, |lint| {
   |                              ^^^^ method not found in `&LateContext<'_>`
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
LL | | }
   |
   |
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:34:28
   |
LL |                           cx.lint(CRATE_NOT_OKAY, |lint| {
   |                              ^^^^ method not found in `&LateContext<'_>`
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
LL | | }
   |
   |
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:34:28
   |
LL |                           cx.lint(CRATE_NOT_OKAY, |lint| {
   |                              ^^^^ method not found in `&LateContext<'_>`
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
LL | | }
   |
   |
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/issue-40001.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:19:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:22:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'tcx>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:53:16
   |
LL |             cx.lint(MISSING_ALLOWED_ATTR, |lint| {
   |                ^^^^ method not found in `&LateContext<'tcx>`
error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/lint-group-denied-lint-allowed.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};
   |                                             ^^^^^^^^^  ^^^^^^^^^^^          ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:37:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:26:28
   |
LL |             "lintme" => cx.lint(TEST_LINT, |lint| {
   |                            ^^^^ method not found in `&LateContext<'_>`

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:29:34
   |
LL |             "pleaselintme" => cx.lint(PLEASE_LINT, |lint| {
   |                                  ^^^^ method not found in `&LateContext<'_>`
error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};
   |                                             ^^^^^^^^^  ^^^^^^^^^^^          ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:37:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:26:28
   |
LL |             "lintme" => cx.lint(TEST_LINT, |lint| {
   |                            ^^^^ method not found in `&LateContext<'_>`

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:29:34
   |
LL |             "pleaselintme" => cx.lint(PLEASE_LINT, |lint| {
   |                                  ^^^^ method not found in `&LateContext<'_>`
error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/lint-group-forbid-always-trumps-cli.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-forbid-always-trumps-cli/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};
   |                                             ^^^^^^^^^  ^^^^^^^^^^^          ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:37:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:26:28
   |
LL |             "lintme" => cx.lint(TEST_LINT, |lint| {
   |                            ^^^^ method not found in `&LateContext<'_>`

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:29:34
   |
LL |             "pleaselintme" => cx.lint(PLEASE_LINT, |lint| {
   |                                  ^^^^ method not found in `&LateContext<'_>`
error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};
   |                                             ^^^^^^^^^  ^^^^^^^^^^^          ^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:37:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:26:28
   |
LL |             "lintme" => cx.lint(TEST_LINT, |lint| {
   |                            ^^^^ method not found in `&LateContext<'_>`

error[E0599]: no method named `lint` found for reference `&LateContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:29:34
   |
LL |             "pleaselintme" => cx.lint(PLEASE_LINT, |lint| {
   |                                  ^^^^ method not found in `&LateContext<'_>`
error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:32:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:25:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:71
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass};

error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:47:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:35:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`

error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:40:16
   |
LL |             cx.lint(TEST_GROUP, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:79
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

error: aborting due to 2 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintContext`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:47
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};


warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:47:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:35:16
   |
LL |             cx.lint(TEST_LINT, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`

error[E0599]: no method named `lint` found for reference `&EarlyContext<'_>` in the current scope
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:40:16
   |
LL |             cx.lint(TEST_GROUP, |lint| {
   |                ^^^^ method not found in `&EarlyContext<'_>`
warning: unused import: `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:79
   |
   |
LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintId, LintPass};

error: aborting due to 2 previous errors; 4 warnings emitted

For more information about this error, try `rustc --explain E0599`.
---
test result: FAILED. 51 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:05

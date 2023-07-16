plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 68 tests
...F...........F................F.FF....FFFFFFFFF...F..FFFFF......F.
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
normalized stderr:
normalized stderr:
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_data_structures;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_middle;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | use rustc_macros::HashStable;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.

---
To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_data_structures;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_middle;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | use rustc_macros::HashStable;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.

---
normalized stderr:
error[E0658]: compiler plugins are deprecated
  --> $DIR/feature-gate-plugin.rs:4:1
   |
LL | #![plugin(empty_plugin)]
   |
   = note: see issue #29597 <https://github.com/rust-lang/rust/issues/29597> for more information
   = note: see issue #29597 <https://github.com/rust-lang/rust/issues/29597> for more information
   = help: add `#![feature(plugin)]` to the crate attributes to enable

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(empty_plugin)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted

---
To only update this specific test, also pass `--test-args feature-gate-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/feature-gate-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/feature-gate-plugin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: compiler plugins are deprecated
  --> /checkout/src/test/ui-fulldeps/feature-gate-plugin.rs:4:1
   |
LL | #![plugin(empty_plugin)]
   |
   = note: see issue #29597 <https://github.com/rust-lang/rust/issues/29597> for more information
   = note: see issue #29597 <https://github.com/rust-lang/rust/issues/29597> for more information
   = help: add `#![feature(plugin)]` to the crate attributes to enable

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(empty_plugin)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted


For more information about this error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_for_crate)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: crate is not marked with #![crate_okay]
   |
   |
LL | / #![feature(plugin)]
LL | | #![plugin(lint_for_crate)]
LL | |
LL | |
LL | | pub fn main() { }
   |
   |
   = note: requested on the command line with `-D crate-not-okay`
error: aborting due to previous error; 1 warning emitted



---
To only update this specific test, also pass `--test-args issue-15778-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_for_crate)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: crate is not marked with #![crate_okay]
   |
   |
LL | / #![feature(plugin)] //~ ERROR crate is not marked with #![crate_okay]
LL | | #![plugin(lint_for_crate)]
LL | | //~^ WARN use of deprecated attribute `plugin`
LL | |
LL | | pub fn main() { }
   |
   |
   = note: requested on the command line with `-D crate-not-okay`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/issue-76270-panic-in-libproc-macro.rs stdout ----
normalized stderr:
error: proc macro panicked
  --> $DIR/issue-76270-panic-in-libproc-macro.rs:15:1
   |
LL | proc_macro_panic::panic_in_libproc_macro!();
   |
   |
   = help: message: `""` is not a valid identifier
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args issue-76270-panic-in-libproc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs:15:1
   |
LL | proc_macro_panic::panic_in_libproc_macro!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: `""` is not a valid identifier
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_for_crate_rpass)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issue-15778-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_for_crate_rpass)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: `-D test-lint` implied by `-D lint-me`

error: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() { }
   |
   |
   = note: `-D please-lint` implied by `-D lint-me`
error: aborting due to 2 previous errors; 1 warning emitted



---
To only update this specific test, also pass `--test-args lint-group-plugin-deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   |
   = note: `-D test-lint` implied by `-D lint-me`

error: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() { } //~ ERROR item is named 'pleaselintme'
   |
   |
   = note: `-D please-lint` implied by `-D lint-me`
error: aborting due to 2 previous errors; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args lint-plugin-cmdline-allow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args lint-plugin-cmdline-load.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "crate-attr=plugin(lint_plugin_test)" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> <crate attribute>:1:1
   |
LL | plugin(lint_plugin_test)
   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/pathless-extern-unstable.rs stdout ----
normalized stderr:
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | pub use rustc_middle;
   |         ^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---
To only update this specific test, also pass `--test-args pathless-extern-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pathless-extern-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pathless-extern-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "rustc_middle" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pathless-extern-unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | pub use rustc_middle;
   |         ^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
note: the lint level is defined here
  --> $DIR/lint-plugin-deny-attr.rs:7:9
   |
   |
LL | #![deny(test_lint)]

error: aborting due to previous error; 1 warning emitted


---
To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
   |
   |
LL | #![deny(test_lint)]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui-fulldeps/issue-40001.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(issue_40001_plugin)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issue-40001.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(issue_40001_plugin)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: `#[warn(test_lint)]` on by default

warning: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() { }
   |
   |
   = note: `#[warn(please_lint)]` on by default
warning: 3 warnings emitted



---
To only update this specific test, also pass `--test-args lint-group-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_group_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default

warning: item is named 'pleaselintme'
   |
   |
LL | fn pleaselintme() { } //~ WARNING item is named 'pleaselintme'
   |
   |
   = note: `#[warn(please_lint)]` on by default
warning: 3 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/plugin-args.rs stdout ----
normalized stderr:
error[E0498]: malformed `plugin` attribute
   |
   |
LL | #![plugin(empty_plugin(args))]
   |           ^^^^^^^^^^^^^^^^^^ malformed attribute

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(empty_plugin(args))]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted

---
To only update this specific test, also pass `--test-args plugin-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0498]: malformed `plugin` attribute
   |
   |
LL | #![plugin(empty_plugin(args))]
   |           ^^^^^^^^^^^^^^^^^^ malformed attribute

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(empty_plugin(args))]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

error: aborting due to previous error; 1 warning emitted


For more information about this error, try `rustc --explain E0498`.

------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: requested on the command line with `-D test-lint`
error: aborting due to previous error; 1 warning emitted



---
To only update this specific test, also pass `--test-args lint-plugin-deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   |
   = note: requested on the command line with `-D test-lint`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
normalized stderr:
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {}
   |
note: the lint level is defined here
  --> $DIR/lint-plugin-forbid-attrs.rs:7:11
   |
   |
LL | #![forbid(test_lint)]


error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0453`.

---
To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() {} //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
   |
   |
LL | #![forbid(test_lint)]


error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #![forbid(test_lint)]
   |           --------- `forbid` level set here
...
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0453`.


------------------------------------------


---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
normalized stderr:
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: requested on the command line with `-F test-lint`

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)]
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0453`.

---
To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) incompatible
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) incompatible
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
   |
   = note: requested on the command line with `-F test-lint`

error[E0453]: allow(test_lint) incompatible with previous forbid
   |
   |
LL | #[allow(test_lint)] //~ ERROR allow(test_lint) incompatible
   |         ^^^^^^^^^ overruled by previous forbid
   |
   = note: `forbid` lint level was set on command line
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0453`.


------------------------------------------


---- [ui] ui-fulldeps/outlive-expansion-phase.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(outlive_expansion_phase)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args outlive-expansion-phase.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/outlive-expansion-phase.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(outlive_expansion_phase)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted



---
To only update this specific test, also pass `--test-args lint-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_plugin_test)] //~ WARNING use of deprecated attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ WARNING item is named 'lintme'
   |
   |
   = note: `#[warn(test_lint)]` on by default
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
normalized stderr:
warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: item is named 'lintme'
   |
   |
LL | fn lintme() {}
   |
   |
   = note: `#[warn(clippy::test_lint)]` on by default

warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`
warning: 6 warnings emitted



---
To only update this specific test, also pass `--test-args lint-tool-cmdline-allow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`

warning: item is named 'lintme'
   |
   |
LL | fn lintme() {}
   |
   |
   = note: `#[warn(clippy::test_lint)]` on by default

warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
   |
   = note: requested on the command line with `-A test_lint`
warning: 6 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
normalized stderr:
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> $DIR/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
   = note: `#[warn(renamed_and_removed_lints)]` on by default


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: unknown lint: `this_lint_does_not_exist`
   |
   |
LL | #[deny(this_lint_does_not_exist)]
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> $DIR/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> $DIR/lint-tool-test.rs:9:23
  --> $DIR/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

error: item is named 'lintme'
   |
   |
LL | fn lintme() { }
   |
note: the lint level is defined here
  --> $DIR/lint-tool-test.rs:13:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`

error: item is named 'lintmetoo'
   |
   |
LL |     fn lintmetoo() { }
   |
note: the lint level is defined here
  --> $DIR/lint-tool-test.rs:13:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
error: aborting due to 2 previous errors; 11 warnings emitted



---
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
   = note: `#[warn(renamed_and_removed_lints)]` on by default


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: unknown lint: `this_lint_does_not_exist`
   |
   |
LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`

error: item is named 'lintmetoo'
   |
   |
LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
error: aborting due to 2 previous errors; 11 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lto-syntax-extension.rs stdout ----
normalized stderr:
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lto_syntax_extension_plugin)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args lto-syntax-extension.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lto-syntax-extension.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lto_syntax_extension_plugin)] //~ WARNING compiler plugins are deprecated
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
test result: FAILED. 47 passed; 21 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:29

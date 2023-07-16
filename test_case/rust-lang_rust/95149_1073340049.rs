plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 67 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...F.....................................................F.........

---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
diff of stderr:


44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
45    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to 5 previous errors
- error: aborting due to 5 previous errors
+ error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
+    |
+    |
+ LL | #[derive(HashStable)]
+    |
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
+    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
+    |
+    |
+ LL | #[derive(HashStable)]
+    |
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
+    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
+    |
+    |
+ LL | #[derive(HashStable)]
+    |
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
+    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
+    |
+    |
+ LL | #[derive(HashStable)]
+    |
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
+    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
+    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 9 previous errors
48 
49 For more information about this error, try `rustc --explain E0658`.
50 
---
To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary"
stdout: none
--- stderr -------------------------------
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
LL | extern crate rustc_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_query_system;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
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

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
diff of stderr:

83 LL |     fn lintmetoo() { }
85    |
- note: the lint level is defined here
-   --> $DIR/lint-tool-test.rs:14:9
-    |
-    |
- LL | #![deny(clippy_group)]
-    |         ^^^^^^^^^^^^
91    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
92 
93 warning: lint name `test_group` is deprecated and may not have an effect in the future.

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
stdout: none
--- stderr -------------------------------
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
   |
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
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:36:8
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
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:14:9
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
   |
   = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
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
error: aborting due to 2 previous errors; 14 warnings emitted
------------------------------------------



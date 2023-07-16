plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 72 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......................F.i.......................................F.......

---- [ui] src/test/ui-fulldeps/gated-plugin.rs stdout ----
diff of stderr:


1 error[E0658]: compiler plugins are deprecated
-   --> $DIR/gated-plugin.rs:3:1
+   --> $DIR/gated-plugin.rs:4:1
3    |
4 LL | #![plugin(empty_plugin)]


8    = help: add `#![feature(plugin)]` to the crate attributes to enable
9 
10 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/gated-plugin.rs:3:1
+   --> $DIR/gated-plugin.rs:4:1
12    |
13 LL | #![plugin(empty_plugin)]
14    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/gated-plugin.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args gated-plugin.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/gated-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: compiler plugins are deprecated
   |
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


---- [ui] src/test/ui-fulldeps/multiple-plugins.rs stdout ----
diff of stderr:

1 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/multiple-plugins.rs:8:1
3    |
3    |
4 LL | #![plugin(multiple_plugins_1)]
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
7    = note: `#[warn(deprecated)]` on by default
8 
8 
9 warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
-   --> $DIR/multiple-plugins.rs:9:1
11    |
11    |
12 LL | #![plugin(multiple_plugins_2)]
13    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/multiple-plugins/multiple-plugins.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args multiple-plugins.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/multiple-plugins.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/multiple-plugins/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/multiple-plugins/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(multiple_plugins_1)] //~ WARN use of deprecated attribute `plugin`
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(multiple_plugins_2)] //~ WARN use of deprecated attribute `plugin`
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
warning: 2 warnings emitted
------------------------------------------



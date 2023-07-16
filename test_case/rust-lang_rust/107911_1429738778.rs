
failures:

---- [ui] tests/ui/invalid_macro_export_argument.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/home/alejandra/git/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/alejandra/git/rust/tests/ui/invalid_macro_export_argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/alejandra/git/rust/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/alejandra/git/rust/build/x86_64-unknown-linux-gnu/test/ui/invalid_macro_export_argument" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/alejandra/git/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/alejandra/git/rust/build/x86_64-unknown-linux-gnu/test/ui/invalid_macro_export_argument/auxiliary"
stdout: none
--- stderr -------------------------------
warning: `#[macro_export]` can only take 1 or 0 arguments
  --> fake-test-src-base/invalid_macro_export_argument.rs:1:1
   |
LL | #[macro_export(hello, world)] //~ WARN `#[macro_export]` can only take 1 or 0 arguments
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(invalid_macro_export_arguments)]` on by default

warning: `not_local_inner_macros` isn't a valid `#[macro_export]` argument
  --> fake-test-src-base/invalid_macro_export_argument.rs:6:16
   |
LL | #[macro_export(not_local_inner_macros)] //~ WARN `not_local_inner_macros` isn't a valid `#[macro_export]` argument
   |                ^^^^^^^^^^^^^^^^^^^^^^

warning: 2 warnings emitted
------------------------------------------



failures:
    [ui] tests/ui/invalid_macro_export_argument.rs

test result: FAILED. 4 passed; 1 failed; 14429 ignored; 0 measured; 0 filtered out; finished in 0.23s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:00:02

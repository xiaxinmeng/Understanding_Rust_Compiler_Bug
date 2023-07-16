plain

---- [ui] tests/ui/parser/builtin-syntax.rs stdout ----
diff of stderr:

10 LL |     builtin#{}();
12 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0658]: builtin# syntax is experimental
+    |
+    |
+ LL |     builtin#foobar();
+    |
+    = note: see issue #110680 <https://github.com/rust-lang/rust/issues/110680> for more information
+    = help: add `#![feature(builtin_syntax)]` to the crate attributes to enable
14 
---
To only update this specific test, also pass `--test-args parser/builtin-syntax.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/builtin-syntax.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/builtin-syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/builtin-syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown `builtin#` construct `foobar`
  --> fake-test-src-base/parser/builtin-syntax.rs:2:5
   |
LL |     builtin#foobar(); //~ ERROR unknown `builtin#` construct


error: expected identifier after `builtin#`
  --> fake-test-src-base/parser/builtin-syntax.rs:6:13
   |
LL |     builtin#{}(); //~ ERROR expected identifier after


error[E0658]: builtin# syntax is experimental
  --> fake-test-src-base/parser/builtin-syntax.rs:2:13
   |
LL |     builtin#foobar(); //~ ERROR unknown `builtin#` construct
   |
   = note: see issue #110680 <https://github.com/rust-lang/rust/issues/110680> for more information
   = help: add `#![feature(builtin_syntax)]` to the crate attributes to enable


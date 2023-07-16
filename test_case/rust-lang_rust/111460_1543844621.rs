plain
- error[E0435]: attempt to use a non-constant value in a constant
+ error: attempt to use a non-constant value in a constant
2   --> $DIR/issue-3668.rs:8:34
3    |
4 LL |        static childVal: Box<P> = self.child.get();
-    |        ---------------           ^^^^ non-constant value
-    |        |
-    |        |
-    |        help: consider using `let` instead of `static`: `let childVal`
+    |                                  ^^^^ help: try using `Self`
9 error: aborting due to previous error
10 

- For more information about this error, try `rustc --explain E0435`.
---
To only update this specific test, also pass `--test-args issues/issue-3668.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-3668.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3668" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3668/auxiliary"
stdout: none
error: attempt to use a non-constant value in a constant
  --> fake-test-src-base/issues/issue-3668.rs:8:34
   |
   |
LL |        static childVal: Box<P> = self.child.get();
   |                                  ^^^^ help: try using `Self`
error: aborting due to previous error
------------------------------------------



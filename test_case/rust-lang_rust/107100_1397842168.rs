plain
-   --> $DIR/issue-107087.rs:13:1
+ error[E0223]: ambiguous associated type
+   --> $DIR/issue-107087.rs:16:5
3    |
- LL | impl Foo for A<u32> {
- LL |     type B = i32;
- LL | }
- LL | }
-    | - ...matches this closing brace
- LL | }
-    | ^ unexpected closing delimiter
-    | ^ unexpected closing delimiter
+ LL |     A::B::<>::C
+    |     ^^^^^^^^ help: use the fully-qualified path: `<A<_> as Foo>::B`
13 error: aborting due to previous error
14 

+ For more information about this error, try `rustc --explain E0223`.
---
To only update this specific test, also pass `--test-args typeck/issue-107087.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-107087.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-107087" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-107087/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0223]: ambiguous associated type
   |
   |
LL |     A::B::<>::C
   |     ^^^^^^^^ help: use the fully-qualified path: `<A<_> as Foo>::B`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0223`.
------------------------------------------

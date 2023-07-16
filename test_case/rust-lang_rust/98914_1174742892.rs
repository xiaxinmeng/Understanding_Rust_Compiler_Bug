plain
.....................iii................................................................ 13112/13152
........................................
failures:

---- [ui] src/test/ui/deref-patterns/gate.rs stdout ----

1 error[E0308]: mismatched types
-   --> $DIR/gate.rs:3:9
+   --> $DIR/gate.rs:4:9
---
To only update this specific test, also pass `--test-args deref-patterns/gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deref-patterns/gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-patterns/gate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-patterns/gate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/deref-patterns/gate.rs:4:9
   |
LL |     match String::new() {
   |           ------------- this expression has type `String`

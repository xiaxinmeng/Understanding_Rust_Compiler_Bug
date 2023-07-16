plain
diff of stderr:

10    |             ++++    +
11 
12 error[E0072]: recursive type `Foo` has infinite size
-   --> $DIR/infinite-struct.rs:9:1
14    |
15 LL | struct Foo {
16    | ^^^^^^^^^^

---
To only update this specific test, also pass `--test-args infinite/infinite-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-struct/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0072]: recursive type `Take` has infinite size
   |
   |
LL | struct Take(Take);
   | ^^^^^^^^^^^ ---- recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
   |
LL | struct Take(Box<Take>);


error[E0072]: recursive type `Foo` has infinite size
   |
   |
LL | struct Foo { //~ ERROR has infinite size
   | ^^^^^^^^^^
LL |     x: Bar<Foo>,
   |            --- recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
   |
LL |     x: Bar<Box<Foo>>,
   |            ++++   +
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0072`.
------------------------------------------

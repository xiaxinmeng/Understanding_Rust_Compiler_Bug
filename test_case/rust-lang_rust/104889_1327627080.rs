plain
 - Checking "/checkout/src/librustdoc/html/static/css/themes/ayu.css"... OK
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 189 tests
........................................................i.......................F.F..... 88/189
.............
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [ui] src/test/rustdoc-ui/infinite-recursive-type-impl-trait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/infinite-recursive-type-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type-impl-trait" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type-impl-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0072]: recursive type `f::E` has infinite size
   |
LL |     enum E {
   |     ^^^^^^
LL |         V(E),
LL |         V(E),
   |           - recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
   |
LL |         V(Box<E>),
   |           ++++ +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.
------------------------------------------
------------------------------------------


---- [ui] src/test/rustdoc-ui/infinite-recursive-type-impl-trait-return.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/infinite-recursive-type-impl-trait-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type-impl-trait-return" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/infinite-recursive-type-impl-trait-return/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0072]: recursive type `f::{closure#0}::E` has infinite size
   |
LL |     enum E {
   |     ^^^^^^
LL |         This(E),
LL |         This(E),
   |              - recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
   |
LL |         This(Box<E>),
   |              ++++ +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.
------------------------------------------

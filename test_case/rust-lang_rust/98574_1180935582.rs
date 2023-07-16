plain
---- [ui] src/test/ui/let-else/let-else-bindings.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/let-else/let-else-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-bindings/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-bindings/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of `foo.0`, as `foo` is a captured variable in an `Fn` closure
   |
   |
LL |     let foo = Foo::Three("three".to_string(), 42);
   |         --- captured outer variable
LL |     let three = || {
LL |     let three = || {
   |                 -- captured by this `Fn` closure
LL |         let Foo::Three(s, _x) = foo else {
   |                        |
   |                        |
   |                        move occurs because `foo.0` has type `String`, which does not implement the `Copy` trait
   |                        help: consider borrowing here: `&s`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------

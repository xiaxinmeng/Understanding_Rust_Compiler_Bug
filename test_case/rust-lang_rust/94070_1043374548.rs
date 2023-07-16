plain
normalized stderr:
error[E0311]: the parameter type `T` may not live long enough
  --> $DIR/issue-91139.rs:17:12
   |
LL | fn foo<T>() {
   |        - help: consider adding an explicit lifetime bound...: `T: 'a`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args generic-associated-types/issue-91139.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-91139.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0311]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/generic-associated-types/issue-91139.rs:17:12
   |
LL | fn foo<T>() {
   |        - help: consider adding an explicit lifetime bound...: `T: 'a`
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
error: aborting due to previous error


------------------------------------------

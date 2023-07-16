plain
---- [ui] src/test/ui/const-generics/issues/issue-105037.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-105037.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-105037/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-105037/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issues/issue-105037.rs:32:19
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     process_table(Table2::<D>);
   |     ------------- ^^^^^^^^^^^ expected struct `Table1`, found struct `Table2`
   |     arguments to this function are incorrect
   |
   |
   = note: expected struct `Table1<D>`
              found struct `Table2<D>`
  --> /checkout/src/test/ui/const-generics/issues/issue-105037.rs:20:4
   |
   |
LL | fn process_table<T: Table<D>, const D: usize>(_table: T)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

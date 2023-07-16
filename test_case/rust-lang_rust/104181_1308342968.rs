plain
diff of stderr:

7 help: try using a fully qualified path to specify the expected types
8    |
9 LL |     <Qqq as MyTrait<T>>::lol::<[closure@$DIR/suggest-fully-qualified-closure.rs:21:11: 21:13]>(&q, ||());
+    |     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ~
11 
12 error[E0283]: type annotations needed
13   --> $DIR/suggest-fully-qualified-closure.rs:21:7
13   --> $DIR/suggest-fully-qualified-closure.rs:21:7

26 help: try using a fully qualified path to specify the expected types
27    |
28 LL |     <Qqq as MyTrait<T>>::lol::<[closure@$DIR/suggest-fully-qualified-closure.rs:21:11: 21:13]>(&q, ||());
+    |     ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ~
30 
31 error: aborting due to 2 previous errors
32 
---
To only update this specific test, also pass `--test-args traits/suggest-fully-qualified-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-fully-qualified-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-fully-qualified-closure/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-closure.rs:21:7
   |
   |
LL |     q.lol(||());
   |
help: try using a fully qualified path to specify the expected types
   |
   |
LL |     <Qqq as MyTrait<T>>::lol::<[closure@/checkout/src/test/ui/traits/suggest-fully-qualified-closure.rs:21:11: 21:13]>(&q, ||());

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/suggest-fully-qualified-closure.rs:21:7
   |
   |
LL |     q.lol(||());
   |
   |
note: multiple `impl`s satisfying `Qqq: MyTrait<_>` found
   |
   |
LL | impl MyTrait<u32> for Qqq{
...
...
LL | impl MyTrait<u64> for Qqq{
help: try using a fully qualified path to specify the expected types
   |
   |
LL |     <Qqq as MyTrait<T>>::lol::<[closure@/checkout/src/test/ui/traits/suggest-fully-qualified-closure.rs:21:11: 21:13]>(&q, ||());

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.

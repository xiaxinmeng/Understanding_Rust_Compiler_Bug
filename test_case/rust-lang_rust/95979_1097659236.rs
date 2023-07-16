plain

+ error: cannot implement trait on type alias impl trait
+   --> $DIR/coherence-with-closure.rs:10:24
+    |
+ LL | impl Trait for Wrapper<OpaqueClosure> {}
+    |
+ note: type alias impl trait defined here
+   --> $DIR/coherence-with-closure.rs:3:22
+    |
+    |
+ LL | type OpaqueClosure = impl Sized;
+ 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
1 error[E0119]: conflicting implementations of trait `Trait` for type `Wrapper<OpaqueClosure>`
2   --> $DIR/coherence-with-closure.rs:11:1


6 LL | impl<T: Sync> Trait for Wrapper<T> {}
7    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Wrapper<OpaqueClosure>`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
10 
11 For more information about this error, try `rustc --explain E0119`.
---
To only update this specific test, also pass `--test-args coherence/coherence-with-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-with-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-with-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-with-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot implement trait on type alias impl trait
   |
   |
LL | impl Trait for Wrapper<OpaqueClosure> {}
   |
note: type alias impl trait defined here
  --> /checkout/src/test/ui/coherence/coherence-with-closure.rs:3:22
   |
   |
LL | type OpaqueClosure = impl Sized;


error[E0119]: conflicting implementations of trait `Trait` for type `Wrapper<OpaqueClosure>`
   |
   |
LL | impl Trait for Wrapper<OpaqueClosure> {}
   | ------------------------------------- first implementation here
LL | impl<T: Sync> Trait for Wrapper<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Wrapper<OpaqueClosure>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0119`.
------------------------------------------

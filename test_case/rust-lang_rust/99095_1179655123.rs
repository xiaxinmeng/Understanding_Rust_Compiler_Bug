plain
diff of stderr:

8    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `i32`
9    |
10    = note: upstream crates may add a new impl of trait `std::iter::Iterator` for type `i32` in future versions
-    = note: upstream crates may add a new impl of trait `std::iter::Iterator` for type `i32` in future versions
13 error: aborting due to previous error
14 


---
To only update this specific test, also pass `--test-args coherence/coherence-projection-conflict-orphan.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-projection-conflict-orphan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-projection-conflict-orphan" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-projection-conflict-orphan/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `Foo<i32>` for type `i32`
   |
   |
LL | impl Foo<i32> for i32 { }
   | --------------------- first implementation here
LL |
LL | impl<A:Iterator> Foo<A::Item> for A { }
   |
   |
   = note: upstream crates may add a new impl of trait `std::iter::Iterator` for type `i32` in future versions
error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
------------------------------------------

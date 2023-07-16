plain

---- [ui] src/test/ui/suggestions/inner_type2.rs stdout ----
diff of stderr:

17 LL |     item.method();
18    |          ^^^^^^ method not found in `MaybeUninit<Struct<u32>>`
19    |
-    = help: if this `MaybeUninit::<Struct<u32>>` has been initialized, use one of the `assume_init` methods to access the inner value
+    = help: if this `MaybeUninit<Struct<u32>>` has been initialized, use one of the `assume_init` methods to access the inner value
21 note: the method `method` exists on the type `Struct<u32>`
23    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type2/inner_type2.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/inner_type2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/inner_type2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/inner_type2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `method` found for struct `LocalKey` in the current scope
   |
   |
LL |     STRUCT.method();
   |            ^^^^^^ method not found in `LocalKey<Struct<u32>>`
   |
   = help: use `with` or `try_with` to access thread local storage
note: the method `method` exists on the type `Struct<u32>`
   |
LL |     pub fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^^^^^


error[E0599]: no method named `method` found for union `MaybeUninit` in the current scope
   |
   |
LL |     item.method();
   |          ^^^^^^ method not found in `MaybeUninit<Struct<u32>>`
   |
   = help: if this `MaybeUninit<Struct<u32>>` has been initialized, use one of the `assume_init` methods to access the inner value
note: the method `method` exists on the type `Struct<u32>`
   |
LL |     pub fn method(&self) {}
   |     ^^^^^^^^^^^^^^^^^^^^


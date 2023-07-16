plain

---- [ui] src/test/ui/type_length_limit.rs stdout ----
diff of stderr:

4 LL | pub fn drop<T>(_x: T) {}
6    |
6    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
8    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
9 
10 error: reached the type-length limit while instantiating `<[closure@std::rt::lang_start<()...e<()>>::call_once - shim(vtable)`
11   --> $SRC_DIR/core/src/ops/function.rs:LL:COL

13 LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
15    |
15    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
17    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type_length_limit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
stdout: none
--- stderr -------------------------------
error: reached the type-length limit while instantiating `std::mem::drop::<Option<((((...,....., ...), ..., ...), ..., ...)>>`
   |
   |
LL | pub fn drop<T>(_x: T) {}
   |
   |
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'

error: reached the type-length limit while instantiating `<[closure@std::rt::lang_start<()...e<()>>::call_once - shim(vtable)`
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |
   |
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'
error: aborting due to 2 previous errors
------------------------------------------



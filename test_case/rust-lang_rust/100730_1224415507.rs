
---- [ui] src/test/ui/type_length_limit.rs stdout ----
diff of stderr:

4       LL | pub fn drop<T>(_x: T) {}
5          | ^^^^^^^^^^^^^^^^^^^^^
6          |
-          = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
8          = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+          = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
9
10      error: reached the type-length limit while instantiating `<[closure@std::rt::lang_start<()...e<()>>::call_once - shim(vtable)`
11        --> $SRC_DIR/core/src/ops/function.rs:LL:COL

13      LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
14         |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
15         |
-          = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
17         = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+          = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
18
19      error: aborting due to 2 previous errors
20

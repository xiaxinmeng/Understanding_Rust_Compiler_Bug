diff
-/// This different meanings have led to [discussion]s about the ability to make
-/// `unsafe` operations inside `unsafe fn`.
+/// As of now (Rust 1.44), `unsafe fn` can act like an `unsafe` block around
+/// the code inside the function **or** a signal to callers that some conditions
+/// must be met before using it.
+///
+/// Mixing these two meanings can be confusing and [discussion]s exist to use
+/// `unsafe` blocks inside such functions when making `unsafe` operations.

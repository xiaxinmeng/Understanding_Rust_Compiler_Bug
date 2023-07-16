diff
-/// This different meanings have led to [discussion]s about the ability to make
-/// `unsafe` operations inside `unsafe fn`.
+/// As of now (Rust 1.44), this meanings become entwined when writing an
+/// `unsafe fn`: this syntax acts like an `unsafe` block around the code inside
+/// the function **and** as a signal to callers about the conditions they must
+/// met before using it.
+///
+/// Mixing this two meanings as one can be confusing and [discussion]s exist
+/// about the necessity to use `unsafe` blocks inside such functions when making
+/// `unsafe` operations.

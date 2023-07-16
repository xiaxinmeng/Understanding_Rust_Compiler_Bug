
-   45|      1|//! fn some_func() {
-   46|      1|//!     println!("called some_func()");
-   47|      1|//! }
+   45|       |//! fn some_func() {
+   46|       |//!     println!("called some_func()");
+   47|       |//! }
    48|       |//!
    49|       |//! #[derive(Debug)]
    50|       |//! struct SomeError;
    51|       |//!
    52|       |//! extern crate doctest_crate;
    53|       |//!
-   54|      1|//! fn doctest_main() -> Result<(), SomeError> {
-   55|      1|//!     some_func();
-   56|      1|//!     doctest_crate::fn_run_in_doctests(2);
-   57|      1|//!     Ok(())
-   58|      1|//! }
+   54|       |//! fn doctest_main() -> Result<(), SomeError> {
+   55|       |//!     some_func();
+   56|       |//!     doctest_crate::fn_run_in_doctests(2);
+   57|       |//!     Ok(())
+   58|       |//! }
    59|       |//!
    60|       |//! // this `main` is not shown as covered, as it clashes with all the other
    61|       |//! // `main` functions that were automatically generated for doctests
@@ -70,9 +66,9 @@
    66|       |
    67|       |/// doctest attached to fn testing external code:
    68|       |/// 
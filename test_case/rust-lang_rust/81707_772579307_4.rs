
-   23|      2|//! #[derive(Debug, PartialEq)]
-                       ^1
-   24|      1|//! struct SomeError {
-   25|      1|//!     msg: String,
-   26|      1|//! }
-   27|      1|//! let mut res = Err(SomeError { msg: String::from("a message") });
-   28|      1|//! if res.is_ok() {
-   29|      0|//!     res?;
+   22|       |//! 
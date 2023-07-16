diff
diff --git a/library/core/src/prelude/v1.rs b/library/core/src/prelude/v1.rs
index c89fe57cb05..04eec513091 100644
--- a/library/core/src/prelude/v1.rs
+++ b/library/core/src/prelude/v1.rs
@@ -29 +29 @@
-pub use crate::convert::{AsMut, AsRef, From, Into};
+pub use crate::convert::{AsMut, AsRef, From, Into, TryFrom, TryInto};
@@ -38 +38 @@
-pub use crate::iter::{Extend, IntoIterator, Iterator};
+pub use crate::iter::{Extend, IntoIterator, Iterator, FromIterator};
diff --git a/library/std/src/prelude/v1.rs b/library/std/src/prelude/v1.rs
index 4a3c3ba1635..f548c42d991 100644
--- a/library/std/src/prelude/v1.rs
+++ b/library/std/src/prelude/v1.rs
@@ -23 +23 @@
-pub use crate::convert::{AsMut, AsRef, From, Into};
+pub use crate::convert::{AsMut, AsRef, From, Into, TryFrom, TryInto};
@@ -29 +29 @@
-pub use crate::iter::{Extend, IntoIterator, Iterator};
+pub use crate::iter::{Extend, IntoIterator, Iterator, FromIterator};

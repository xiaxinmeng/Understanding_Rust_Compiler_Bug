
-    #[unstable(feature = "slice_split_at_unchecked", reason = "new API", issue = "76014")]
     #[inline]
     unsafe fn split_at_mut_unchecked(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
         let len = self.len();
diff --git a/library/core/src/str/validations.rs b/library/core/src/str/validations.rs
index 373a8212425..56d8506d9a4 100644
--- a/library/core/src/str/validations.rs
+++ b/library/core/src/str/validations.rs
@@ -250,9 +250,8 @@ pub(super) fn run_utf8_validation(v: &[u8]) -> Result<(), Utf8Error> {
 ];
 
 /// Given a first byte, determines how many bytes are in this UTF-8 character.
-#[unstable(feature = "str_internals", issue = "none")]
 #[inline]
-pub fn utf8_char_width(b: u8) -> usize {
+pub(crate) fn utf8_char_width(b: u8) -> usize {
     UTF8_CHAR_WIDTH[b as usize] as usize
 }
 
diff --git a/library/core/src/task/poll.rs b/library/core/src/task/poll.rs
index fc0a4e74797..5cc050e97be 100644
--- a/library/core/src/task/poll.rs
+++ b/library/core/src/task/poll.rs
@@ -1,5 +1,3 @@
-#![stable(feature = "futures_api", since = "1.36.0")]
-
 use crate::convert;
 use crate::ops::{self, ControlFlow};
 use crate::result::Result;
diff --git a/library/core/src/task/wake.rs b/library/core/src/task/wake.rs
index b775e022a54..f6740ea928c 100644
--- a/library/core/src/task/wake.rs
+++ b/library/core/src/task/wake.rs
@@ -1,5 +1,3 @@
-#![stable(feature = "futures_api", since = "1.36.0")]
-
 use crate::fmt;
 use crate::marker::{PhantomData, Unpin};

diff
diff --git a/src/lib.rs b/src/lib.rs
index 132b5c8..3a3d321 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -103,7 +103,7 @@ impl<'a, R: Read, A: Automaton<&'a [u8]> > Iterator for SplitByIter<'a, R, A> {
 /// second
 /// ********########
 /// third
-/// ################
+/// #################
 /// last"#.split_by(&ac);
 ///
 /// assert!(splits.next().unwrap().unwrap().as_slice() == b"first\n");

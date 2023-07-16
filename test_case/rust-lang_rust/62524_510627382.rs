diff
diff --git a/src/libsyntax_pos/span_encoding.rs b/src/libsyntax_pos/span_encoding.rs
index 525ec13623..d7da2206ab 100644
--- a/src/libsyntax_pos/span_encoding.rs
+++ b/src/libsyntax_pos/span_encoding.rs
@@ -74,6 +74,12 @@ pub const DUMMY_SP: Span = Span { base_or_index: 0, len_or_tag: 0, ctxt_or_zero:
 impl Span {
     #[inline]
     pub fn new(mut lo: BytePos, mut hi: BytePos, ctxt: SyntaxContext) -> Self {
+        if lo == BytePos(10) || hi == BytePos(10) {
+            eprintln!("=========================");
+            eprintln!("{:?}", (lo, hi));
+            eprintln!();
+            let _ = std::panic::catch_unwind(|| panic!()); // potentially print backtrace
+        }
         if lo > hi {
             std::mem::swap(&mut lo, &mut hi);
         }

 diff
--- old.rs 2014-03-30 20:40:18.290040065 -0500
+++ new.rs     2014-03-30 20:47:11.096850380 -0500
@@ -26 +26 @@
-fn encode_json<'a, T: Encodable<json::Encoder<'a>>>(val: &T, wr: &'a mut MemWriter) {
+fn encode_json<'a, T: Encodable<json::Encoder<'a>, std::io::IoError>>(val: &T, wr: &'a mut MemWriter) {
@@ -30 +30 @@
-fn encode_ebml<'a, T: Encodable<writer::Encoder<'a>>>(val: &T, wr: &'a mut MemWriter) {
+fn encode_ebml<'a, T: Encodable<writer::Encoder<'a, MemWriter>, std::io::IoError>>(val: &T, wr: &'a mut MemWriter) {

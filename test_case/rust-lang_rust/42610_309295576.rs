
--- a/src/proto.rs
+++ b/src/proto.rs
@@ -305,7 +305,6 @@ impl NewPacket {
                 let length = u24_le(&*self.header).unwrap();
                 self.last_seq_id = self.header[3];
                 debug!("Last seq id {}", self.last_seq_id);
-                self.header.clear();
                 if length == 0 {
                     return ParseResult::Done(Packet { payload: self.data }, self.last_seq_id);
                 } else {

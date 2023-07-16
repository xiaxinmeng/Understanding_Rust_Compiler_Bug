diff
-     self.start = if byte_offset >= mem::size_of::<T>() {
+     self.start = if intrinsics::likely(byte_offset >= mem::size_of::<T>()) {

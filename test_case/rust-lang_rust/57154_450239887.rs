diff
diff --git a/src/cloudabi/mod.rs b/src/cloudabi/mod.rs
index df11de00..dba78bd9 100644
--- a/src/cloudabi/mod.rs
+++ b/src/cloudabi/mod.rs
@@ -9,6 +9,11 @@ pub type uint16_t = u16;
 pub type uint32_t = u32;
 pub type uint64_t = u64;
 
+pub type c_char = i8;
+pub type c_long = i32;
+pub type c_ulong = u32;
+pub type wchar_t = u16;
+
 pub type c_schar = i8;
 pub type c_uchar = u8;
 pub type c_short = i16;

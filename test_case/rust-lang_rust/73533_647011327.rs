diff
diff --git a/src/libcore/fmt/num.rs b/src/libcore/fmt/num.rs
index 7d77e33d743..1457da8c667 100644
--- a/src/libcore/fmt/num.rs
+++ b/src/libcore/fmt/num.rs
@@ -187,10 +187,10 @@ static DEC_DIGITS_LUT: &[u8; 200] = b"0001020304050607080910111213141516171819\
       8081828384858687888990919293949596979899";
 
 macro_rules! impl_Display {
-    ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
+    ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident max $max_len:literal) => {
         fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-            // 2^128 is about 3*10^38, so 39 gives an extra byte of space
-            let mut buf = [MaybeUninit::<u8>::uninit(); 39];
+            // 2^bits is about $max_len, so 39 gives an extra byte of space
+            let mut buf = [MaybeUninit::<u8>::uninit(); $max_len];
             let mut curr = buf.len() as isize;
             let buf_ptr = MaybeUninit::first_ptr_mut(&mut buf);
             let lut_ptr = DEC_DIGITS_LUT.as_ptr();
@@ -198,28 +198,30 @@ macro_rules! impl_Display {
             // SAFETY: Since `d1` and `d2` are always less than or equal to `198`, we
             // can copy from `lut_ptr[d1..d1 + 1]` and `lut_ptr[d2..d2 + 1]`. To show
             // that it's OK to copy into `buf_ptr`, notice that at the beginning
-            // `curr == buf.len() == 39 > log(n)` since `n < 2^128 < 10^39`, and at
-            // each step this is kept the same as `n` is divided. Since `n` is always
-            // non-negative, this means that `curr > 0` so `buf_ptr[curr..curr + 1]`
-            // is safe to access.
+            // `curr == buf.len() == $max_len > log(n)` since `n < 2^128 < 10^bits`,
+            // and at each step this is kept the same as `n` is divided. Since `n`
+            // is always non-negative, this means that `curr > 0` so
+            // `buf_ptr[curr..curr + 1]` is safe to access.
             unsafe {
                 // need at least 16 bits for the 4-characters-at-a-time to work.
                 assert!(crate::mem::size_of::<$u>() >= 2);
 
                 // eagerly decode 4 characters at a time
-                while n >= 10000 {
-                    let rem = (n % 10000) as isize;
-                    n /= 10000;
-
-                    let d1 = (rem / 100) << 1;
-                    let d2 = (rem % 100) << 1;
-                    curr -= 4;
-
-                    // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
-                    // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
-                    // which is `10^40 > 2^128 > n`.
-                    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
-                    ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
+                if $max_len > 4 {
+                    while n >= 10000 {
+                        let rem = (n % 10000) as isize;
+                        n /= 10000;
+
+                        let d1 = (rem / 100) << 1;
+                        let d2 = (rem % 100) << 1;
+                        curr -= 4;
+
+                        // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
+                        // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
+                        // which is `10^40 > 2^128 > n`.
+                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
+                        ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
+                    }
                 }
 
                 // if we reach here numbers are <= 9999, so at most 4 chars long
@@ -440,10 +442,10 @@ macro_rules! impl_Exp {
 #[cfg(any(target_pointer_width = "64", target_arch = "wasm32"))]
 mod imp {
     use super::*;
-    impl_Display!(
-        i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
-            as u64 via to_u64 named fmt_u64
-    );
+    impl_Display!(i8, u8 as u64 via to_u64 named fmt_u8 max 3);
+    impl_Display!(i16, u16 as u64 via to_u64 named fmt_u16 max 5);
+    impl_Display!(i32, u32 as u64 via to_u64 named fmt_u32 max 10);
+    impl_Display!(i64, u64, usize, isize as u64 via to_u64 named fmt_u64 max 20);
     impl_Exp!(
         i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
             as u64 via to_u64 named exp_u64
@@ -453,11 +455,13 @@ mod imp {
 #[cfg(not(any(target_pointer_width = "64", target_arch = "wasm32")))]
 mod imp {
     use super::*;
-    impl_Display!(i8, u8, i16, u16, i32, u32, isize, usize as u32 via to_u32 named fmt_u32);
-    impl_Display!(i64, u64 as u64 via to_u64 named fmt_u64);
+    impl_Display!(i8, u8 as u32 via to_u32 named fmt_u8 max 3);
+    impl_Display!(i16, u16 as u32 via to_u32 named fmt_u16 max 5);
+    impl_Display!(i32, u32, isize, usize as u32 via to_u32 named fmt_u32 max 10);
+    impl_Display!(i64, u64 as u64 via to_u64 named fmt_u64 max 20);
     impl_Exp!(i8, u8, i16, u16, i32, u32, isize, usize as u32 via to_u32 named exp_u32);
     impl_Exp!(i64, u64 as u64 via to_u64 named exp_u64);
 }
 
-impl_Display!(i128, u128 as u128 via to_u128 named fmt_u128);
+impl_Display!(i128, u128 as u128 via to_u128 named fmt_u128 max 39);
 impl_Exp!(i128, u128 as u128 via to_u128 named exp_u128);

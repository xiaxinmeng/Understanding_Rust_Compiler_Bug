 diff
diff --git a/src/libcore/num/bignum.rs b/src/libcore/num/bignum.rs
index dac7193..ca3419b 100644
--- a/src/libcore/num/bignum.rs
+++ b/src/libcore/num/bignum.rs
@@ -104,11 +104,7 @@ impl_full_ops! {

 /// Table of powers of 5 representable in digits. Specifically, the largest {u8, u16, u32} value
 /// that's a power of five, plus the corresponding exponent. Used in `mul_pow5`.
-const SMALL_POW5: [(u64, usize); 3] = [
-    (125, 3),
-    (15625, 6),
-    (1_220_703_125, 13),
-];
+const SMALL_POW5: [(u64, usize); 3] = [(125, 3), (15625, 6), (1_220_703_125, 13)];

 macro_rules! define_bignum {
     ($name:ident: type=$ty:ty, n=$n:expr) => (
diff --git a/src/libcore/num/diy_float.rs b/src/libcore/num/diy_float.rs
index 4a8332b..01fbcdc 100644
--- a/src/libcore/num/diy_float.rs
+++ b/src/libcore/num/diy_float.rs
@@ -40,7 +40,7 @@ impl Fp {
         let bc = b * c;
         let ad = a * d;
         let bd = b * d;
-        let tmp = (bd >> 32) + (ad & MASK) + (bc & MASK) + (1 << 31) /* round */;
+        let tmp = (bd >> 32) + (ad & MASK) + (bc & MASK) + (1 << 31);
         let f = ac + (ad >> 32) + (bc >> 32) + (tmp >> 32);
         let e = self.e + other.e + 64;
         Fp { f: f, e: e }
diff --git a/src/libcore/num/flt2dec/strategy/grisu.rs b/src/libcore/num/flt2dec/strategy/grisu.rs
index e026f3c..dcc3980 100644
--- a/src/libcore/num/flt2dec/strategy/grisu.rs
+++ b/src/libcore/num/flt2dec/strategy/grisu.rs
@@ -144,14 +144,14 @@ pub fn max_pow10_no_more_than(x: u32) -> (u8, u32) {
     debug_assert!(x > 0);

     const X9: u32 = 10_0000_0000;
-    const X8: u32 =  1_0000_0000;
-    const X7: u32 =    1000_0000;
-    const X6: u32 =     100_0000;
-    const X5: u32 =      10_0000;
-    const X4: u32 =       1_0000;
-    const X3: u32 =         1000;
-    const X2: u32 =          100;
-    const X1: u32 =           10;
+    const X8: u32 = 1_0000_0000;
+    const X7: u32 = 1000_0000;
+    const X6: u32 = 100_0000;
+    const X5: u32 = 10_0000;
+    const X4: u32 = 1_0000;
+    const X3: u32 = 1000;
+    const X2: u32 = 100;
+    const X1: u32 = 10;

     if x < X4 {
         if x < X2 {

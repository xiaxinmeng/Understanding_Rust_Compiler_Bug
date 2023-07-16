 diff
     #[inline]
     fn is_infinite(self) -> bool {
-        self == Float::infinity() || self == Float::neg_infinity()
+        self == f32::infinity() || self == f32::neg_infinity()
     }

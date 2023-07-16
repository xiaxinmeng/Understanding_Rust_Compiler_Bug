diff
-        dst[index] &= src1[index] < src2[index];
+        dst[index] = dst[index] as u8 & (src1[index] < src2[index]) as u8 != 0;

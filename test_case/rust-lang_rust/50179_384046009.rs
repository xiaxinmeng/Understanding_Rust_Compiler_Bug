diff
 pub fn mod_euc(self, rhs: f64) -> f64 {
     let mut r = self % rhs;
     if r < 0.0 {
         r += rhs.abs();
-        if r == rhs {
+        if r == rhs.abs() {
             return 0.0;
         }
     }
     r
 }

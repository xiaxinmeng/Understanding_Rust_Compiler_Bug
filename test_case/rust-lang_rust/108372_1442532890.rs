diff
 pub fn variant_X(input: &[(usize, usize, usize, usize)]) -> usize {
     input
         .iter()
-        .filter(|(a, b, c, d)| a <= c && d <= b || c <= a && b <= d)
+        .filter(|(a, b, c, d)| *a <= *c && *d <= *b || *c <= *a && *b <= *d)
         .count()
 }

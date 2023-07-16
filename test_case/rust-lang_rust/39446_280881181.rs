diff
--- main.rs.orig	2017-02-18 14:41:31.993231078 -0800
+++ main.rs	2017-02-18 14:43:25.862194979 -0800
@@ -43,8 +43,8 @@ fn main() {
         y_y = 0;
         i = 0;
         while (i < max_iter && x_x + y_y <= 800) {
-          x_x = (x * x) / 200;
-          y_y = (y * y) / 200;
+          x_x = ((x * x) as u32 / 200) as i32;
+          y_y = ((y * y) as u32 / 200) as i32;
           if (x_x + y_y > 800) {
             the_char = 48 + i;
             if (i > 9) {
@@ -53,9 +53,9 @@ fn main() {
           } else {
             temp = x_x - y_y + x0;
             if ((x < 0 && y > 0) || (x > 0 && y < 0)) {
-              y = (-1 * ((-1 * (x * y)) / 100)) + y0;
+              y = (-1 * ((-1 * (x * y)) as u32 / 100) as i32) + y0;
             } else {
-              y = x * y / 100 + y0;
+              y = ((x * y) as u32 / 100) as i32 + y0;
             }
             x = temp;
           }

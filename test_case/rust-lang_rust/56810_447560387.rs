diff
@@ -58,7 +58,7 @@
         switchInt(move _6) -> [false: bb10, otherwise: bb1];
     }
     bb10: {
-        falseEdges -> [real: bb11, imaginary: bb5];
+        falseEdges -> [real: bb6, imaginary: bb5];
     }
     bb11: {
         _7 = Le(const 6i32, _1);

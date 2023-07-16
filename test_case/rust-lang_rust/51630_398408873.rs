diff
-        self.borrow.set(if borrow == MIN_WRITING {
-            UNUSED
-        } else {
-            borrow - 1
-        });
+        self.borrow.set(borrow + 1);

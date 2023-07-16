diff
@@ -46,16 +46,16 @@ v is 0 (match), construct is if
   fn(&self):    drop,body,
 v is 1 (mismatch), construct is let else
   vanilla:      drop,else,
-  &:            else,drop,
-  &mut:         else,drop,
+  &:            drop,else,
+  &mut:         drop,else,
   move:         drop,else,
   tuple:        drop,else,
   array:        drop,else,
   nested:       drop,inner,else,
   fn(this):     drop,else,
   fn(&self):    drop,else,
-  ref &:        else,drop,
-  ref mut &mut: else,drop,
+  ref &:        drop,else,
+  ref mut &mut: drop,else,
 v is 1 (mismatch), construct is if let
   vanilla:      else,drop,
   &:            else,drop,

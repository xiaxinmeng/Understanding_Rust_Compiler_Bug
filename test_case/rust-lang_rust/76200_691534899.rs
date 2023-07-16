diff
-        let [] = [(); align_of::<Self>() - align_of::<*mut T>()];
+        let [] = [(); align_of::<AtomicPtr<()>>() - align_of::<*mut ()>()];

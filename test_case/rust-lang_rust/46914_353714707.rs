diff
-141        let arr = &mut self.values as &mut [ManuallyDrop<_>];
+141        let arr = &mut self.values as &mut [ManuallDrop<<A as Array>::Element>];
...
-263        let arr = &mut source_array_vec.values as &mut [ManualyDrop<_>];
+263        let arr =
+264            &mut source_array_vec.values as &mut [ManuallyDrop<<A as Array>::Element>];

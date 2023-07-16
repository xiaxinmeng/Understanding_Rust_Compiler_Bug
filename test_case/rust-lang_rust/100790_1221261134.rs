patch
- struct Example
- where
-     (): Sized,
- (usize);
+ struct Example(usize)
+ where
+     (): Sized;

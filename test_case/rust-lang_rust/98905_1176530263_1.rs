diff
-pub fn open<P: AsRef<Path>>(path: P) -> Result<File>`
+pub fn open<P: ?Sized + AsRef2<Path>>(path: &P) -> Result<File>

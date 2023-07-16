diff
impl<'global> Result<'global> {
    fn run(
        &self,
-        _callback: impl for <'a> FnMut(&dyn Trait<'global, 'a>)
+        _callback: impl for <'a> FnMut(&'a dyn Trait<'global, 'a>)
    ) {
        todo!()
    }
}

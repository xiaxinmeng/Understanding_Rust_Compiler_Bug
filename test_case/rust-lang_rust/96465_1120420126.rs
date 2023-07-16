diff
impl<T> Trait for Struct<T> {
    type Gat<'this> = &'this mut T where Self: 'this;

    fn function<F>(self, _: F)
    where
-        F: FnOnce(Self::Gat<'_>),
+        F: FnOnce(&mut T),
    {
    }
}

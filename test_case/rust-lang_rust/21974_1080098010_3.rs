rust
impl<'outer: 'outer, 'inner: 'inner> MyTrait for &'outer &'inner Baz {}

rust
impl<'a, T> ConsistentRef for &'a T
    where T: Consistent + ?Sized

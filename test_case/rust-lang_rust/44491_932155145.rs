rust
fn normalize<T: Integer>(a: T, n: &T) -> T
    where for<'a> &'a T: IntegerOps<T>
{ ... }

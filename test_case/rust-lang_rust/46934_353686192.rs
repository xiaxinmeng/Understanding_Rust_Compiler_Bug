rust
trait Contains<T>
where
    T: ?Sized,
{
    fn contains<U>(&self, x: &U) -> bool
    where
        T: PartialEq<U>,
        U: ?Sized,
    {
        true
    }
}

impl<'a, T> Contains<T> for &'a [T] {}

#[test]
fn test() {
    let a: &str = "string";
    let v: &[&str] = &[a];
    Contains::contains(&v, a);
}

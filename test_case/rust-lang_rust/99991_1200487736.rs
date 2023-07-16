rust
trait Func<T> {}
impl<T, F> Func<T> for F where F: Fn(T) {}

fn test<F>(_: F)
where
    for<'a> F: Func<&'a ()>,
{
}

fn main() {
    let f: fn(&()) = |t| {};
    test(f); //works
    test(|t| {}); //error
}

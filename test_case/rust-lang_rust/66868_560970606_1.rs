
pub fn g<T>(task: T)
where
    T: Send,
{
}

fn main() {
    g(foo::f());
}

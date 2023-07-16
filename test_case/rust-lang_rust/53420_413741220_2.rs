rust
trait Lt<'a> {
    type T;
}
struct Id<T>(T);
impl<'a,T> Lt<'a> for Id<T> {
    type T = T;
}

fn main() {
    let f: fn(_) = |_:<Id<()> as Lt<'_>>::T| {};
    f(())
}

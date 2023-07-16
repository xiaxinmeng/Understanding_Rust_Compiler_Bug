rs
trait Trait<'a, T> {
    fn m();
}

impl<'a, A> Trait<'a, A> for () {
    fn m() {}
}

fn main() {
    <() as Trait<'static, _>>::m();
}

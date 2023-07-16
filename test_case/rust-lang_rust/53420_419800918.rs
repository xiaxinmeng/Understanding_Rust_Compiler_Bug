rust
trait Lt<'a> {
    type T;
}
impl<'a> Lt<'a> for () {
    type T = ();
}

fn main() {
    let _:fn(()) = |_:<() as Lt<'_>>::T| {};
}

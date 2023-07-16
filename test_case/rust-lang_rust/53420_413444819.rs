rust
trait Lt<'a> {
    type T;
}
impl<'a> Lt<'a> for () {
    type T = ();
}

fn main() {
    let _:fn(<() as Lt<'_>>::T) = |()| {};
}

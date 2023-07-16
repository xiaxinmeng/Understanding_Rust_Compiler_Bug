rust
trait Lt<'a> {
    type T;
}
impl<'a> Lt<'a> for () {
    type T = ();
}

fn test<'a>() {
    let _:fn(<() as Lt<'a>>::T) = |()| {};
}

fn main() {
    test();
}

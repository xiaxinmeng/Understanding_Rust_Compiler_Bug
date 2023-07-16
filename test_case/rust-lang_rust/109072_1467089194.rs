`rust
trait Lt<'_> {
    type T = ();
}
impl<> Lt<>  () {}

fn main() {
    let v:<() as Lt<>>::T = ();
}

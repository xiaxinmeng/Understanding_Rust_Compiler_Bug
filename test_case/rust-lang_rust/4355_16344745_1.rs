
trait A {
    fn a(~self);
}

impl A for int {
    fn a(~self) { }
}

fn main() {
    (~1).a();
}

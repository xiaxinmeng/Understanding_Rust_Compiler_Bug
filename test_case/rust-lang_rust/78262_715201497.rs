rust
trait TT {}

impl (dyn TT + '_) {
    fn func(&self) {}
}

fn main() {
    let _f = |x: &dyn TT| x.func();
}

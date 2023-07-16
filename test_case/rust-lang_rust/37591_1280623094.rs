rust
trait Assoc {
    type Ty;
}

impl<T> dyn Assoc<Ty = T> {
    fn non_method() {}
}

fn main() {
    <dyn Assoc>::non_method()
}

 rust
trait Y<'a>: Sized {
    fn g() where Self: for<'r> Y<'r> {
    }
}

fn main() {}

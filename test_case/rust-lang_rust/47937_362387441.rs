rust
trait Trait2 where Self: Sized {
    fn f(&self) {
        let _ = self as &Trait2;
    }
}

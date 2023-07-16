rust
trait Trait2 {
    fn f(&self) where Self: Sized {
        let _ = self as &Trait2;
    }
}

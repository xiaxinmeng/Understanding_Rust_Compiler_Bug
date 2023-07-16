rust
trait T {
    fn f(&self, _: ()) {
        None::<()>.map(Self::f);
    }
}

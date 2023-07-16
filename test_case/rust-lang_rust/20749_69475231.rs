 rust
trait Add {
    type Output;
    fn add(self) -> <Self as Add>::Output;
}

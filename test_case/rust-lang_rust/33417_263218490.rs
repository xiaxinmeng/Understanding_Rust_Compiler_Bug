rust
impl TryInto<X> for Y {
    type Err = MyErrorType;

   fn try_into(self) -> Result<X, Self::Err> { ... }
}

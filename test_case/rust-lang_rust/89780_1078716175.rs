rust
trait Try {
    type Output;
    type Residual: Residual<Self::Output, TryType = Self>;
    ...
}

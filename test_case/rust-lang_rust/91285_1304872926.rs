rust
trait Residual {
    type TryType<O>: Try<Output = O, Residual = Self>;
}

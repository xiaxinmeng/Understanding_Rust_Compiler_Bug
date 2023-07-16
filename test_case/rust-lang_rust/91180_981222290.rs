rust
trait A: Iterator<Item = Self::X> {
    type X;
}

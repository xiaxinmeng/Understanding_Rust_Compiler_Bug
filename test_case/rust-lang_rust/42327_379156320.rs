rust
trait TryContinue {
    type Continue;
    fn from_continue(_: Self::Continue) -> Self;
}

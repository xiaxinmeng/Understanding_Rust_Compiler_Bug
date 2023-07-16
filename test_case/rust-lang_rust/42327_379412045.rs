rust
trait TryContinue {
  type Continue;
  fn from_continue(_: Self::Continue) -> Self;
}

trait Try<E>: TryContinue {
  fn try(self) -> Result<Self::Continue, E>
}

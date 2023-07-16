
trait TypeEq {
  static fn eq<A: TypeEq, B: TypeEq>() -> bool;
}

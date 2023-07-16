rust
enum Foo {
  Unresumed(A, B),
  YieldedOnce(A, B, X),
  YieldedTwice(A, B, Y, Z),
  YieldedThrice(A, B, Z),
}

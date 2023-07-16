rust
trait X: Send {}

type A = X;
type B = X + Send;

// A == B ?

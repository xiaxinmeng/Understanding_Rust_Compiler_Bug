 rust
trait One { type Foo: Two; }
trait Two { type Foo: One; }

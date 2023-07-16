 rust
trait Foo: TotalEq { fn method(&self); }

struct ShouldntBeTotalEq;
impl Foo for ShouldntBeTotalEq { fn method(&self) {} }

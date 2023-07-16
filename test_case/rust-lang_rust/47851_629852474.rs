rust
struct Foo;

// Does not prevent the warning
trait Bar0 {}
impl Bar0 for Foo {}

// Does not prevent the warning
trait Bar1 { fn dummy(); }
impl Bar1 for Foo { fn dummy () {} }

// Prevents the warning
trait Bar2 { fn dummy(&self); }
impl Bar2 for Foo { fn dummy (&self) {} }

// Prevents the warning
trait Bar3 { fn dummy() -> Self; }
impl Bar3 for Foo { fn dummy () -> Self { todo!() } }

// Prevents the warning
trait Bar4 { type Dummy; }
impl Bar4 for Foo { type Dummy = Self; }

// Prevents the warning
trait Bar5 { const DUMMY: Self; }
impl Bar5 for Foo { const DUMMY: Self = Self; }

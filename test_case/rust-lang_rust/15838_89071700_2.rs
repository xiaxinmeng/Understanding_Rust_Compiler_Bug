 rust
trait Foo<'a> { fn foo(&self) -> &'a u32; }
struct Bar1<'b>(&'b u32, &'b u32);
impl Foo for Bar1 { fn foo(&self) -> &u32 { self.0 } }
fn main() { }

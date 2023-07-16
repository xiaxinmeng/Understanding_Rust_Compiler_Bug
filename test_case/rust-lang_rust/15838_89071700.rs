 rust
trait Foo<'a> { fn foo(&self) -> &'a u32; }
struct Bar0(&'static u32, &'static u32);
impl Foo for Bar0 { fn foo(&self) -> &u32 { self.0 } }
fn main() { }

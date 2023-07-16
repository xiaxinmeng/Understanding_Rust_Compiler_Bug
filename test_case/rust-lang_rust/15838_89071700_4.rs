 rust
trait Foo<'a> { fn foo(&self) -> &'a u32; }
struct Bar2<'b,'c>(&'b u32, &'c u32);
impl Foo for Bar2 { fn foo(&self) -> &u32 { self.0 } }
fn main() { }

 rust
trait methods         { fn foo(&self) -> ~str;            }
impl methods for uint { fn foo(&self) -> ~str { ~"uint" } }
impl methods for int  { fn foo(&self) -> ~str { ~"int"  } }

fn main() {
    assert!(22.foo() == ~"int");
}

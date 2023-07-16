 rust
trait Fooable { fn foo(&self) -> int; }
impl Fooable for int { fn foo(&self) -> int { 3 } }
impl Fooable for @str { fn foo(&self) -> int { 4 } }

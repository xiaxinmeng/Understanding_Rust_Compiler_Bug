rust
trait Write {}

impl <'a,T> Write for &'a mut std::vec::Vec<T> where &'a mut T:Write {}
impl <'a> Write for &'a mut u8 {}

fn foo<T: Write>(_arg:T) {}

fn main() {
    let mut v = vec![];
    foo(&mut v);
}

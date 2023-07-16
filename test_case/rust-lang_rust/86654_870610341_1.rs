rust
fn a() { println!("a"); }
fn b() { println!("b"); }

trait Foo { fn foo(&mut self); }
impl Foo for fn() {
    fn foo(&mut self) {
        *self = b;
    }
}

fn main() {
    let mut fn_ptr = a as fn();
    (&mut fn_ptr as &mut dyn Foo).foo();
    fn_ptr(); // prints "b"
}

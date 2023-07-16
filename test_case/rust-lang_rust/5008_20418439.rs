 rust
trait Debuggable {
    fn debug_name(&self) -> ~str;
}

struct Thing {
    name: ~str,
}

impl Thing {
    fn new() -> Thing { Thing { name: ~"dummy" } }
}

impl Debuggable for Thing {
    fn debug_name(&self) -> ~str { copy self.name }
}

fn print_name(x: &Debuggable) {
    println(fmt!("debug_name = %s", x.debug_name()));
}

fn main() {
    let thing = Thing::new();
    print_name(&thing as &Debuggable);
}

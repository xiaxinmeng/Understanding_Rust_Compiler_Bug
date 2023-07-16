
trait Trait {
    fn method(&self) { println!("doing"); }
}

impl Trait for fn() {}

fn function() {}

fn main() {
    // This doesn't work
    function.method();

    // This does
    let f: fn() = function;
    f.method();
}

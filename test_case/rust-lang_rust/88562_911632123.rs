rs
trait Trait {}

impl Trait for fn(&'static ()) {}

fn main() {
    let f: fn(&()) = |_| ();
    let _x = &f as &fn(&'static ()) as &dyn Trait;
}

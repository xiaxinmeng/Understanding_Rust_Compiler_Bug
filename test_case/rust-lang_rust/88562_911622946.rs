rs
trait Trait {
    fn method(&self);
}

impl<'a> Trait for fn(&'a ()) {
    fn method(&self) {}
}

struct Struct(fn(&()));

impl Trait for Struct {
    fn method<'a>(&'a self) {
        let _x = &self.0 as &fn(&'a ()) as &dyn Trait;
    }
}

fn main() {
    let _x = Trait::method as fn(&Struct);
}

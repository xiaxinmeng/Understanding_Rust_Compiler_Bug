 rust
trait Vehicle { fn stuff(&self) {} }
struct Car;
impl Vehicle for Car { }

fn build_car<'a>() -> Box<Vehicle + 'a> {
    loop {
        return Box::new(Car);
    }
}

fn main() { }

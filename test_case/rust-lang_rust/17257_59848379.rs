
trait Vehicle { }
struct Car;
impl Vehicle for Car { }

fn build_car<'a>() -> Box<Vehicle + 'a> {
    loop {
        return box Car;
    }
}

fn main() { }

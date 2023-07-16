
struct Car;
trait Vehicle { }
impl Vehicle for Car { }

fn build_car<'a>() -> Box<Vehicle + 'a> {
    return box Car;
}

fn main() { }

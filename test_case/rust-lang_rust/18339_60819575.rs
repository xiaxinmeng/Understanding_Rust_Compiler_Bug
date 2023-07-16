 rust
use std::fmt;

enum OptionalInt {
    Value(int),
    Missing,
}

impl fmt::Show for OptionalInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value(i) => write!(f, "Value({})", i),
            Missing  => write!(f, "Missing"),
        }
    }
}

fn main() {
    let x = Value(6i);

    match x {
        Value(i) if i > 5 => println!("Got an int bigger than five from {}!", x),
        Value(..) => println!("Got an int from {}!", x),
        Missing   => println!("No such luck: {}.", x),
    }
}

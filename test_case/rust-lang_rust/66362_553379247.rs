rust
enum Animal {
    Spider { iHateSnakes: bool },
}

fn main() {
    let pet = Animal::Spider { iHateSnakes: false };

    match pet {
        Animal::Spider { iHateSnakes } => dbg!(iHateSnakes),
    };
}

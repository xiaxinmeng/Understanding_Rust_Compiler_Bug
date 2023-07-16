 rust
#[derive(Debug)]
struct Car {
    name: String,
}

#[derive(Debug)]
struct Wheel<'a> {
    size: i32,
    owner: &'a Car,
}

fn main() {
    let car = Car { name: "DeLorean".to_string() };

    let mut wheels = vec!();

    for _ in 0..4 {
        wheels.push(Wheel { size: 360, owner: &car });
    }
}

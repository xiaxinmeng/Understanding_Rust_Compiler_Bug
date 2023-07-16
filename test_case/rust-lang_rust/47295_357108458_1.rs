rust
fn main() {
    let x = Box::new(1);

    let v = (1, 2);

    match v {
        (1, 2) if rand(x) => (),
        (1, 2) if rand(x) => (), // May consume `x` twice
        _ => (),
    }
}

fn rand<T>(_: T) -> bool { false /* or true */}

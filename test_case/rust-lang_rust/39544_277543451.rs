
enum X {
    Y
}

struct Z {
    x: X
}

fn main() {
    let z = Z { x: X::Y };
    let _ = &mut z.x;
}

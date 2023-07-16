
enum AB {
    A {x: int},
    B
}

fn main() {
    let a = B;
    match a {
        A {x : _x} => {fail!()}
        _ => {}
    }
}

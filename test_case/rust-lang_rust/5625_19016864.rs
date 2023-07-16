 rust
enum AB {
    A {x: int},
    B
}

fn main() {
    let a = A { x: 1 };
    let b = B;

    match a {
        A {x : _x1} => {
            match b {
                A {x : _x2} => println("AA"),
                _ => println("A_")
            }
        },
        _ => println("_")
    }
}

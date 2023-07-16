 rust
struct numgroup {
        x : int,
        y : int
}

fn main() {
        let i = numgroup { x:1, y:2 };
        add(i, i);
        add(i, i);
        println!("{}", add(i, i));
}

fn add(x: numgroup, y: numgroup) -> int {
        ((x.x) + (x.y)) + ((y.x) + (y.y))
}

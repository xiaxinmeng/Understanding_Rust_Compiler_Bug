 rust
type V = [int, .. 10000];

fn main() {
    let x: V = [0, .. 10000];

    let y = x, z = x;

    println(fmt!("%? %? %?", x, y, z))
}

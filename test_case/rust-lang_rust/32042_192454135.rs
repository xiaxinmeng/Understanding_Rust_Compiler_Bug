 rust
fn main() {
    let iter = (1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2)
        .chain(1..2);
    for x in iter { println!("{:?}", x); }
}

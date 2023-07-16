
fn main() {
    let iter = (0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2)
        .chain(0..2);
    for i in iter { println!("{:?}", i); }
}

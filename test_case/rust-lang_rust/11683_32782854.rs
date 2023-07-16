 rust
fn main() {
    let (p, c) = Chan::<int>::new();

    do spawn {
        c.send(1);
    }
    println!("{}", p.recv());
}

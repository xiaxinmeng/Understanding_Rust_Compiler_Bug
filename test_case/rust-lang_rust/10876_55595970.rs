
enum Nat {
    S(Box<Nat>),
    Z
}
fn test(x: &mut Nat) {
    let mut p = &mut *x;
    loop {
        match p {
            &Z => break,
            &S(box ref mut n) => p = &mut *n
        }
    }
}

fn main() { }

 rust
enum Ref {
    N(Box<Ref>),
    K(()),
}

fn crash(mut x: &mut Ref) {
    loop {
        match x {
            &mut Ref::N(ref mut n) => { x = n; }
            &mut Ref::K(ref s) => break
        }
    }
}

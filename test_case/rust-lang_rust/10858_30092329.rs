 rust
fn foo(input: &Nat) {
    let mut n = input;
    loop {
        let x = *n;
        n = match x {
            S(~ref m) => m,
            _ => return
        }
    }
}

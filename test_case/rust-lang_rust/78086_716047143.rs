rust
fn main() {
    let a = 1_u8;
    let b = a.into(); // The only in the snippet error occurs here.

    let c = From::from(a);
    take(c);
}

fn take(_: u16) {}

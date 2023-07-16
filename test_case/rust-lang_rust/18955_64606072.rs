 rust
fn bla() {
    let x : Option<int> = None;
    x.unwrap();
}

fn bumm() {
    bip();
}

fn bip() {
    bla();
}

fn main() {
    bumm();
}

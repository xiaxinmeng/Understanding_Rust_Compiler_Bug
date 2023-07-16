rust
extern crate term;

fn foo(t: &mut std::io::Write) {
    t.flush().unwrap();
}

fn main() {
    let mut t = term::stdout().unwrap();
    foo(&mut t);
}

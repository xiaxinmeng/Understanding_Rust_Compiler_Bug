rust
extern crate term;

fn main() {
    let mut t = term::stdout().unwrap();
    writeln!(t, "foo").unwrap();
}

 rust
fn main() {
    let a = "foo bar baz";

    for i in a.split(' ').take(1) { println!("{}", i); }
    for i in a.splitn(' ', 1) {:println!("{}", i); }
}

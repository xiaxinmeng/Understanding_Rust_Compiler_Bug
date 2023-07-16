 rust
fn main() {
    let a = "foo bar baz";

    println!("{}", a.split(' ').take(2).collect::<Vec<&str>>());
    println!("{}", a.splitn(' ', 2).collect::<Vec<&str>>());
}

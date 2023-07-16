 rust
fn f(x: &String) -> &String { f(x) }

fn main() {
    println!("{}", f(&String::from("abc")));
}

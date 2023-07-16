rust
fn foo() -> i32 {
    [1,2,3][42]  // error here
}

fn main() {
    println!("{}", foo()) // panic here
}

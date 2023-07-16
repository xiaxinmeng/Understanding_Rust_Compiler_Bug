rust
fn foo() -> String {
    unimplemented!()
}

fn main() {
    let x = foo().as_str();
    println!("{}", x);
}

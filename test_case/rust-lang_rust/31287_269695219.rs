Rust
fn main() {
    let x = Box::new(0);
    match () {
        _ if { drop(x); false } => {}
        () => println!("{:?}", x)
    }
}

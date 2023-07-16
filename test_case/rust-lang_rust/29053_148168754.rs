 rust
fn main() {
    let x: &'static str = "x";

    {
        let z = &mut &*x;
        *z = "z";
    }

    println!("{:?}", x);
}

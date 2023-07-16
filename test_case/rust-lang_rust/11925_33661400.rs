 rust
fn main() {
    let r = {
        let x = ~42;
        let f = proc() &x;
        f()
    };

    println!("{:?}", r);
}

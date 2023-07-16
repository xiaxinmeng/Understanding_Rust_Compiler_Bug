rust
fn main() {
    let mut x = 0;
    let closure = move || {
        let mut closure2 = move || {
            x += 1;
            x
        };
        closure2();
        println!("{}", x);
    };
    closure();
}

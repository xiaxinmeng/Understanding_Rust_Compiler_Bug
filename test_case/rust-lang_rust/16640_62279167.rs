 rust
fn main() {
    let x = 1i;
    let f = move || {
        move || { x+1 }
    };
    println!("{}", f()());
}

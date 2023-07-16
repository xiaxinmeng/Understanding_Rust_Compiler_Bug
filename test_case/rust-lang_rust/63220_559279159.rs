Rust
fn main() {
    let mut x = 0;
    let mut y = move |z: u32| {
        x += z;
        println!("inner x {}", x);
    };

    println!("outer x {}", x);
    y(2);
    y(2);
    println!("outer x {}", x);
}

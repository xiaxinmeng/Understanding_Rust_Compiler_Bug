rust
fn main() {
    let x = 0;
    let mut y = move |z: u32| {
        let mut x = x + z;
        println!("inner x {}", x);
    };

    println!("outer x {}", x);
    y(2);
    println!("outer x {}", x);
}

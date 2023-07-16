rust
fn weird<T>(v: T) -> T {
    println!("hey");
    v
}

fn main() {
    let mut x = weird(0);
    x = 8;
}

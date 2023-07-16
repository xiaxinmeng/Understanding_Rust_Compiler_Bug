Rust
struct Closure {
  x: u32,
}

impl Closure {
    fn call_mut(&mut self, z: u32){
        self.x += z;
        println!("inner x {}", self.x);
    }
}

fn main() {
    let x = 0;
    let mut y = Closure{x};

    println!("outer x {}", x);
    y.call_mut(2);
    y.call_mut(2);
    println!("outer x {}", x);
}

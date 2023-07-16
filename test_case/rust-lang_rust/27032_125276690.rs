 rust
struct Point { x: i32, y: i32 }

fn main() {
    let mut p = Point { x: 1, y: 3 };
    let borrowed = &mut p;
    println!("{}", p.x);
}

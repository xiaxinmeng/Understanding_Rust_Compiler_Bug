 rust
struct Point { x: u32, y: u32 }
fn main() {
    let mut point: Point = Point { x: 1, y: 2 };
    drop(point);
    point.x = 3;
    println!("{}", point.x); //use of moved value: `point.x`
}

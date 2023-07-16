 rust
struct R { x: i32, y: i32, z: i32 }
fn main() {
    let x = Box::new(0);
    let r = R { x: 0, y: 0, z: 0 };
    let r = R { x: *x, y: { drop(x); let _ = Box::new(main); 0 }, ..r};
    println!("{} {} {}", r.x, r.y, r.z);
}

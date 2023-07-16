rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

main() {
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
}

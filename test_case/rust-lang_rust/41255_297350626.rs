rust
trait Inject<T...; const IN: T> {
    const VALUE: Self;
}

struct Point { x: i32, y: i32 }
impl<const x: i32, const y: i32> Inject<i32, i32; {(x, y)}> for Point {
    const VALUE = Point { x, y };
}

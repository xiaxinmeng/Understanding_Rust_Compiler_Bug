 rust
fn main() {
    fn f1(x: i32, y: i32) -> i32 { y }
    let f: fn(, i32) -> i32 = f1;
    //        ~
    //        |
    //        +--- this is pretty-printed badly.
    f(13);
}

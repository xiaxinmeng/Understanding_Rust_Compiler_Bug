 Rust
trait X {}
impl X for int {}
struct Y<'self>{
    x:Option<&'self X>,
}

fn main() {
    let x = 42;
    let a = Some(&x as &X);
    let _y = Y { x: a };
}


#[derive(Debug)]
struct MyBox<T>(Box<T>);
type FloatBox = MyBox<f32>;

impl FloatBox {
    fn new(x: f32) -> FloatBox {
        MyBox(Box::new(x))
    }
    fn bar() {
        println!("HI")
    }
}

fn main() {
    let foo = FloatBox::new(3.2);
    println!("{:?}", foo);
    FloatBox::bar()
}

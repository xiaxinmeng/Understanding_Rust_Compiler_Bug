 rust
#[derive(Debug)]
pub struct Matrix4<S>(S);
pub trait POrd<S> {}

pub fn translate<S: POrd<S>>(_: S) -> Matrix4<S> { unimplemented!() }

impl POrd<f32> for f32 {}
impl POrd<f64> for f64 {}

fn main() {
    let x = 1.0;
    let m : Matrix4<f32> = translate(x);
    println!("m: {:?}", m);
}

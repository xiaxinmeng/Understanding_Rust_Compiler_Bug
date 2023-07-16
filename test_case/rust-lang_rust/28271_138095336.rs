 rust
#![feature(unboxed_closures)]

pub trait Convert<Input> {
    type Output;

    fn convert(&self, data: Input) -> Self::Output;
}

impl<Input, T> Convert<Input> for T
    where T: Fn<(Input,)>
{
    type Output = T::Output;
    fn convert(&self, data: Input) -> Self::Output {
        (*self)(data)
    }
}

//combines 2 Converts into 1
pub struct Combine<C1, C2>
{
    a: C1,
    b: C2
}

impl<C1, C2, Input> Convert<Input> for Combine<C1, C2>
    where C1: Convert<Input>,
          C2: Convert<C1::Output>
{
    type Output = C2::Output;

    fn convert(&self, data: Input) -> Self::Output {
        let value = self.a.convert(data);
        self.b.convert(value)
    }
}

fn main(){
    let converter = Combine{
        a: |d|{"".to_string()},
        b: |d|{true}
    };
    let value:bool = converter.convert(4);
    println!("value: {:?}", value);
}

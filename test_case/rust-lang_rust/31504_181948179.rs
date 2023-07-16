 rust
use std::marker::PhantomData;

pub trait DataBind {
    type Data;
}

impl<T> DataBind for Global<T> {
    type Data = T;
}

pub struct Global<T>(PhantomData<T>);

pub struct Data {
    pub offsets: <Global<[f32; 1]> as DataBind>::Data,
}

pub fn main() {
    let mut d = Data { offsets: [0.0] };
    d.offsets[0] = 0.0;
}

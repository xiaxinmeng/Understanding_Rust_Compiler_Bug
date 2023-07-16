
#[repr(C)]
pub struct Bar {
    pub a: f32,
    pub b: f32,
    pub _unit: std::marker::PhantomData<()>,
}

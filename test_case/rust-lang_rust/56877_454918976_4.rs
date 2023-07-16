
#[repr(C)]
pub struct Bar {
    pub a: f32,
    pub b: f32,
}

#[repr(transparent)]
pub struct Bar {
    pub c: BarC,
    pub _unit: std::marker::PhantomData<()>,
}

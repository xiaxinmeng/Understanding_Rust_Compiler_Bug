rust
pub trait Resources2 {
    type BufferArray;
}

pub enum RenderCommand2<R: Resources2> {
    BindBuffers { buffers: R::BufferArray },
}

pub struct Ref2;
impl<'a> Resources2 for &'a Ref2 {
    type BufferArray = (&'a u32, &'a u32);
}

pub fn own_render2(com: RenderCommand2<&Ref2>) -> u32 {
    match com {
        RenderCommand2::BindBuffers {
            buffers: (_buffers, _offsets),
        } => 0,
    }
}

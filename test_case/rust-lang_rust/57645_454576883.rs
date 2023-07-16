rust
#[repr(C)]
pub struct MiddleFieldPhantom {
    pub a: f32,
    pub b: f32,
    pub c: [u32; 0],
    pub d: PhantomData<u8>,
}

/// In the case where the VLA is in the middle, but the only thing
/// that comes next is phantomdata, we still consider that to be
/// `has_vla: false`. Not clear if that is correct but also not clear
/// that it is wrong.
#[rustc_layout(abi)]
pub type Test4 = MiddleFieldPhantom;
//~^ ERROR abi: Aggregate { sized: true, has_vla: false }

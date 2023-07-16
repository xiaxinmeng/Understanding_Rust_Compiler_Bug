rust
#[repr(C)]
pub struct __wasi_event_t {
    pub userdata: u64,
    pub error: u16,
    pub type_: u8,
    pub u: T,
}

#[repr(C)]
pub struct Event {
    pub userdata: u64,
    pub res: Result<(), NonZeroU16>,
    pub type_: u8,
    pub u: T,
}

const _ASSERT1: [(); 32] = [(); core::mem::size_of::<__wasi_event_t>()];
const _ASSERT2: [(); 32] = [(); core::mem::size_of::<Event>()];
const _ASSERT3: [(); 0] =  [(); __WASI_ESUCCESS as usize];

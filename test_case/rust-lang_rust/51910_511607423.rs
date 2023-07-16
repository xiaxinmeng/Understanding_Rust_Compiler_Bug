rust
use std::ptr::null;

#[repr(C)]
struct Fields {
    field1: u32,
    field2: [u8; 3],
    field3: u64,
    field4: i32,
}

union Transmuter<T: 'static> {
    ptr: *const T,
    reference: &'static T,
    integer: usize,
}

const FIELD4_OFFSET: usize =
    unsafe { Transmuter { reference: &(Transmuter { ptr: null::<Fields>() }.reference).field4 }.integer };

const _STATIC_ASSERT_EQ_: [(); 16] = [(); FIELD4_OFFSET];

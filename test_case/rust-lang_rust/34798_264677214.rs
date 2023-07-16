rust
use std::marker::PhantomData;

fn main() {
    let buffer = vec![0u8; 16];
    let data = Data {
        ptr: &buffer[0],
        phantom: PhantomData,
    };
    unsafe { bogus(&data) };
}

#[repr(C)]
struct Data<'a> {
    ptr: *const u8,
    phantom: PhantomData<&'a [u8]>,
}

extern "C" {
    fn bogus(data: *const Data);
}

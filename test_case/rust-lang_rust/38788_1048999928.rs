rust
#![no_std]
#![feature(abi_ptx, stdsimd)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[repr(C)]
pub struct Foo{
    array: [f32; 9],
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn add(a: *mut f32, b: Foo) {
    *a = b.array[5];
}

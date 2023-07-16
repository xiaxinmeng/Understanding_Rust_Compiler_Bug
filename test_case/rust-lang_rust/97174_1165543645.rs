rust
#![feature(abi_ptx)]
#![no_std]


#[panic_handler]
unsafe fn breakpoint_panic_handler(_: &::core::panic::PanicInfo) -> ! {
    loop {}
    core::hint::unreachable_unchecked();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreeU8 {
    a: u8,
    b: u8,
    c: u8,
}

// ptx linker is inlining the device function even if it is tagged as `never`
// I have checked that a combination of --emit=llvm-ir actually produces a function 
// in llvm-ir and compiling with llc keeps the functions into ptx assembly.
// TODO: verify that this function is not inlined after ptx-linker is fixed
#[inline(never)]
#[no_mangle]
pub fn device_three_u8(v: ThreeU8) -> ThreeU8 {
    ThreeU8{
        a: v.b,
        b: v.a,
        c: (v.a + v.b)/2,
    }
}

#[inline(never)]
#[no_mangle]
// CHECK: kernel_three_u8
pub unsafe extern "ptx-kernel" fn kernel_three_u8(input: *const ThreeU8, output: *mut ThreeU8) {
    for i in 0..1_000_000 {
        output.write_volatile(device_three_u8(*input));
    }
}

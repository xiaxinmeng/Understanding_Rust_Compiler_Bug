rust
#![allow(dead_code, non_upper_case_globals)]
#![feature(asm)]

//SSE
#[repr(C)]
pub struct D32x4(f32,f32,f32,f32);

impl D32x4 {
    fn add(&self, vec: Self) -> Self {
        unsafe {
            let ret: Self;
            asm!("
                 movaps $1, %xmm1
                 movaps $2, %xmm2
                 addps %xmm1, %xmm2
                 movaps $xmm1, $0
                 "
                 : "=r"(ret)
                 : "1"(self), "2"(vec)
                 : "xmm1", "xmm2"
                 );
            ret
        }
    }
}

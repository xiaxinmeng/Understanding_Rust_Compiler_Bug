 rust
#![allow(dead_code, non_upper_case_globals)]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;

pub mod cpuid;

//SSE
#[repr(C)]
pub struct D32x4(f32,f32,f32,f32);

//SSE2
#[repr(C)]
pub struct D64x2(f64,f64);

#[repr(C)]
pub struct I64x2(i64,i64);
#[repr(C)]
pub struct U64x2(u64,u64);

#[repr(C)]
pub struct I32x4(i32,i32,i32,i32);
#[repr(C)]
pub struct U32x4(u32,u32,u32,u32);

#[repr(C)]
pub struct I16x8(i16,i16,i16,i16,i16,i16,i16,i16);
#[repr(C)]
pub struct U16x8(u16,u16,u16,u16,u16,u16,u16,u16);

#[repr(C)]
pub struct I8x16(i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8,i8);
#[repr(C)]
pub struct U8x16(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8);

pub trait Vector {
    fn add(&self, vec: Self) -> Self;
    fn sub(&self, vec: Self) -> Self;
    fn mul(&self, vec: Self) -> Self;
    fn div(&self, vec: Self) -> Self;
}

impl Vector for D32x4 {
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
                 : "{xmm1}", "{xmm2}"
                 );
            ret
        }
    }

    fn sub(&self, vec: Self) -> Self {
        unsafe {
            let ret: Self;
            asm!("
                 movaps $1, %xmm1
                 movaps $2, %xmm2
                 subps %xmm1, %xmm2
                 movaps $xmm1, $0
                 "
                 : "=r"(ret)
                 : "1"(self), "2"(vec)
                 : "{xmm1}", "{xmm2}"
                 );
            ret
        }
    }

    fn mul(&self, vec: Self) -> Self {
        unsafe {
            let ret: Self;
            asm!("
                 movaps $1, %xmm1
                 movaps $2, %xmm2
                 mulps %xmm1, %xmm2
                 movaps $xmm1, $0
                 "
                 : "=r"(ret)
                 : "1"(self), "2"(vec)
                 : "{xmm1}", "{xmm2}"
                 );
            ret
        }
    }

    fn div(&self, vec: Self) -> Self {
        unsafe {
            let ret: Self;
            asm!("
                 movaps $1, %xmm1
                 movaps $2, %xmm2
                 divps %xmm1, %xmm2
                 movaps $xmm1, $0
                 "
                 : "=r"(ret)
                 : "1"(self), "2"(vec)
                 : "{xmm1}", "{xmm2}"
                 );
            ret
        }
    }
}

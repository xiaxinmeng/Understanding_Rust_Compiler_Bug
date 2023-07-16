
#![crate_type="rlib"]
#![feature(asm)]

#[repr(C, packed)]
pub struct SegmentSelector {
    flags: u16
}

pub unsafe fn load_ss(sel: SegmentSelector) {
    asm!("movw $0, %ss " :: "r" (sel) : "memory");
}

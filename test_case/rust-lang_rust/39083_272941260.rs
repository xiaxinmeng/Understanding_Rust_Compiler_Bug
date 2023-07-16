
pub unsafe fn load_ss(sel: SegmentSelector) {
    asm!("movw $0, %ss " : "+*r" (&sel) :: "memory");
}

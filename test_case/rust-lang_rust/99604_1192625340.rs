rust
fn main() {
    const MARKER: u32 = 0x11223344;
    let mut x = MARKER;
    let mut y = 0_u32;
    unsafe { 
        std::ptr::swap_nonoverlapping(
            &mut x as *mut u32 as *mut (u16, u8), 
            &mut y as *mut u32 as *mut (u16, u8),
            1
        );
    }
    if y != MARKER {
        println!("{y:x} != {MARKER:x}");
    }
}

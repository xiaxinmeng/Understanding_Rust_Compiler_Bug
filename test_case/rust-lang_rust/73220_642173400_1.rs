rust
    fn read_csr(csr_num: usize) -> u32 {
        let r: u32;
        if csr_num == 0x10 {
            unsafe { asm!("csrr {rd}, {csr}", rd = out(reg) r, csr = 0x10) }
        } else if csr_num == 0x20 {
            unsafe { asm!("csrr {rd}, {csr}", rd = out(reg) r, csr = 0x20) }
        }
        r
    }

rust
    fn read_csr(csr_num: usize) -> u32 {
        let r: u32;
        unsafe { asm!("csrr {rd}, {csr}", rd = out(reg) r, csr = csr_num) }
        r
    }

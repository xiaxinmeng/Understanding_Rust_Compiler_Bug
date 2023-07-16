
/// Bytewise slice equality                                                                                                                                                      
#[cfg(test, stage0)]
#[lang="str_eq"]
#[inline]
pub fn eq_slice(a: &str, b: &str) -> bool {
    do a.as_imm_buf |ap, alen| {
        do b.as_imm_buf |bp, blen| {
            if (alen != blen) { false }
            else {
                unsafe {
                    libc::memcmp(ap as *libc::c_void,
                                 bp as *libc::c_void,
                                 (alen - 1) as libc::size_t) == 0
                }
            }
        }
    }
}

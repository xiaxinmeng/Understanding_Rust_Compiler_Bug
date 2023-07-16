rust
pub fn C_big_integral(t: Type, u: u128, sign_extend: bool) -> ValueRef {
    if ::std::mem::size_of::<u128>() == 16 {
        unsafe {
            let words = [u as u64, u.wrapping_shr(64) as u64];
            llvm::LLVMConstIntOfArbitraryPrecision(t.to_ref(), 2, words.as_ptr())
        }
    } else {
        // SNAP: remove after snapshot
        C_integral(t, u as u64, sign_extend)
    }
}

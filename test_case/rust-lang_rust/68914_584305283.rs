rust
    /// Loads up to 7 bytes from a byte-slice into a u64.
    #[inline]
    fn u8to64_le(buf: &[u8], start: usize, len: usize) -> u64 {
        assert!(len <= 8 && start + len <= buf.len());
        let mut out = 0u64;

        unsafe {
            let out_ptr = &mut out as *mut _ as *mut u8;
            ptr::copy_nonoverlapping(buf.as_ptr().offset(start as isize), out_ptr, len);
        }

        #[cfg(target_endian = "big")]
        {
            // If this is a big endian system we swap bytes, so that the first
            // byte ends up in the lowest order byte, like SipHash expects.
            out = out.swap_bytes();
        }

        out
    }

rust
// This feature gate is only used within from_bytes_with_nul_unchecked.
// The same point stands with from_bytes_with_nul returning a unit and removing from_bytes_with_nul_unchecked.
#![feature(const_raw_ptr_deref)]

struct CStr([u8]);

enum CStrConvertError {
    NotNulTerminated,
    InteriorNul,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl CStr {
    pub const fn from_bytes_with_nul(
        bytes: [u8; 5],
        size: usize,
    ) -> Result<&Self, CStrConvertError> {
        if size == 0 {
            return Result::Err(CStrConvertError::NotNulTerminated);
        }
        if bytes[size - 1] != 0 {
            return Result::Err(CStrConvertError::NotNulTerminated);
        }
        let mut i = 0;
        // `i + 1 < bytes.len()` allows LLVM to optimize away bounds checking,
        // while it couldn't optimize away bounds checks for `i < bytes.len() - 1`.
        while i + 1 < size {
            if bytes[i] == 0 {
                return Result::Err(CStrConvertError::InteriorNul);
            }
            i += 1;
        }
        // SAFETY: We just checked that all properties hold.
        Result::Ok(unsafe { Self::from_bytes_with_nul_unchecked(bytes) })
    }

    pub const unsafe fn from_bytes_with_nul_unchecked(bytes: [u8; 5]) -> &CStr {
        // Note: This can be done using pointer deref (which requires
        // `const_raw_ptr_deref` to be const) or `transmute` (which requires
        // `const_transmute` to be const) or `ptr::from_raw_parts` (which
        // requires `ptr_metadata`).
        // While none of them are current stable, it is very likely that one of
        // them will eventually be.
        &*(&bytes as &[u8] as *const [u8] as *const Self)
    }
}

const BYTES: [u8; 5] = [0x77, 'E' as u8, 'L' as u8, 'F' as u8, 0];
const STR: Result<&CStr, CStrConvertError> = CStr::from_bytes_with_nul(BYTES, 5);

rust
fn slice_compare(a: *const i8, b: *const i8, len: usize) -> bool {
    let mut bits = len * 8;
    let mut offset = 0;
    while bits > 0 {
        if bits >= 128 {
            if cmp!(a, b, u128, offset) == false {
                return false;
            }
            bits -= 128;
            offset += 16;
        } else if bits >= 64 {
            if cmp!(a, b, u64, offset) == false {
                return false;
            }
            bits -= 64;
            offset += 8;
        } else if bits >= 32 {
            if cmp!(a, b, u32, offset) == false {
                return false;
            }
            bits -= 32;
            offset += 4;
        } else if bits >= 16 {
            if cmp!(a, b, u16, offset) == false {
                return false;
            }
            bits -= 16;
            offset += 2;
        } else if bits >= 8 {
            if cmp!(a, b, u8, offset) == false {
                return false;
            }
            bits -= 8;
            offset += 1;
        } else {
            unreachable!();
        }
    }
    true
}

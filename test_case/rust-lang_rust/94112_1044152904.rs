rust
use std::intrinsics::transmute;

#[inline]
pub const fn char_try_from_u32_old(i: u32) -> Result<char, ()> {
    if (i > char::MAX as u32) || (i >= 0xD800 && i <= 0xDFFF) {
        Err(())
    } else {
        // SAFETY: checked that it's a legal unicode value
        Ok(unsafe { transmute(i) })
    }
}

#[inline]
pub const fn char_try_from_u32_new(i: u32) -> Result<char, ()> {
    if (i ^ 0xD800).wrapping_sub(0x800) >= 0x110000 - 0x800 {
        Err(())
    } else {
        // SAFETY: checked that it's a legal unicode value
        Ok(unsafe { transmute(i) })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_char_try_from_u32() {
        for i in 0..=u32::MAX {
            assert_eq!(char_try_from_u32_new(i), char_try_from_u32_old(i));
        }
    }
}

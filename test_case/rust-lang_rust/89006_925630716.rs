rust
#![feature(const_panic)]

pub const fn from_u32(i: u32) -> Option<char> {
    if (i > '\u{10ffff}' as u32) || (i >= 0xD800 && i <= 0xDFFF) {
        None
    } else {
        // SAFETY: checked that it's a legal unicode value
        Some(unsafe { core::mem::transmute(i) })
    }
}

pub const unsafe fn from_u32_unchecked(i: u32) -> char {
    if cfg!(debug_assertions) {
        match from_u32(i) {
            Some(val) => val,
            None => panic!("illegal unicode value"),
        }
    } else {
        // SAFETY: checked that it's a legal unicode value
        core::mem::transmute(i)
    }
}

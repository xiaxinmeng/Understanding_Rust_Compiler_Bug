rust
#![no_std]
static VAR: &[u8] = b"42";

pub fn test() -> *const u8 {
    VAR.as_ptr()
}

pub mod nested {
    pub fn test() -> *const u8 {
        super::VAR.as_ptr()
    }
}

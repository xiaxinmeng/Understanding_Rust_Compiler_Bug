rust
#![no_std]
pub struct DeStr;
impl core::ops::Deref for DeStr {
    type Target = str;
    fn deref(&self) -> &str {
        unimplemented!()
    }
}


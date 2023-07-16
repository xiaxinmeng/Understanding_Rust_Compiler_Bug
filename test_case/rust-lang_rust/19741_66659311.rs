 rust
impl ::slice::AsSlice<u8> for str {                
    fn as_slice(&self) -> &[u8] { self.as_bytes() }
}                                                  

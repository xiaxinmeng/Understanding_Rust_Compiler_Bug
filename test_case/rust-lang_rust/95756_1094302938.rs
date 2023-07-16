rust
pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> Result<u64> 
where
    R: Read,
    W: Write, 

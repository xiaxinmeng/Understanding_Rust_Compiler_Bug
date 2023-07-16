rust
pub struct ListFut<'a, 'b>(&'a mut [&'b mut [u8]]);

rust
pub trait Place<Data> 
where
    Data: ?Sized, 
{
    fn pointer(&mut self) -> *mut Data;
}

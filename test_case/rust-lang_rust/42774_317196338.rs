rust
trait Alloc<'a> {
    type DeallocInfo;
    fn dealloc_info(&self) -> Self::DeallocInfo;
    ...
}
impl<'a> Alloc<'a> for &'a MyAllocator {
    type DeallocInfo = MyDeallocInfo<'a>;
    …
}

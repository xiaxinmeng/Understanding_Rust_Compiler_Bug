rust
trait Alloc {
    type DeallocInfo;
    fn dealloc(&self, ptr: *mut T, layout: Layout);
    fn dealloc_with(info: Self::DeallocInfo, ptr: *mut T, layout: Layout);
    // snip
}
struct Rc<T, A: Alloc> {
  dealloc: A::DeallocInfo;
  // snip
}
impl<T> Rc<T, HeapAlloc> {
    fn make_mut(&mut self) -> &mut T { 
         // use HeapAlloc to potentially clone T
    }
}
impl<T, A: Alloc> Rc<T, A> {
    fn make_mut_in(&mut self, a: A) -> &mut T { 
         // use A to potentially clone T
    }
}

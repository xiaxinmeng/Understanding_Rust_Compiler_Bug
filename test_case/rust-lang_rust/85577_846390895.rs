rust
> impl<T: ?Sized, A: Allocator> Box<T, A> {
>     pub fn reuse<U>(b: Box<T, A>, val: U) -> Result<Box<U, A>, (Box<T, A>, U)> { ... }
>     pub fn set<U>(b: Box<T, A>, val: U) -> Box<U, A> { ... }
>     pub fn try_set<U>(b: Box<T, A>, val: U) -> Result<Box<U, A>, AllocError> { ... }
> }
> 
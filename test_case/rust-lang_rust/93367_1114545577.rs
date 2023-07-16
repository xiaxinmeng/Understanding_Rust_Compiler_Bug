rust
> /// A Send + Sync raw pointer wrapper          
> #[derive(Copy, Clone)]
> #[repr(transparent)]
> pub(crate) struct Ptr<T> { ptr: T }
> unsafe impl<T> Sync for Ptr<*const T> { }
> unsafe impl<T> Sync for Ptr<*mut T> { }
> unsafe impl<T> Send for Ptr<*const T> { }
> unsafe impl<T> Send for Ptr<*mut T> { }
> 
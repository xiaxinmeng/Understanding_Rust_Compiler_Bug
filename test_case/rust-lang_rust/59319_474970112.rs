rust
>     let ptr = Box::into_raw(boxed) as *mut Box<Fn()>;
>     let from_raw: Box<Box<Fn()>> = unsafe { Box::from_raw(ptr) };
> 
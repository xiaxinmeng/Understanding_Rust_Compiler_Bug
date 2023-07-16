rust
>     pub unsafe fn transmute_copy<T, U>(src: &T) -> U {
>         ptr::read(src as *const T as *const U)
>     }
>     
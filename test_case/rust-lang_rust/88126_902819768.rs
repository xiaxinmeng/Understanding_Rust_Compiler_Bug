rust
> use std::panic::AssertUnwindSafe;
> 
> struct UnwindUnsafe<'a>(&'a mut u32);
> 
> fn main() {
>     let mut x = 0;
>     let uu = UnwindUnsafe(&mut x);
>     let aus = AssertUnwindSafe(uu);
>     let _ = std::panic::catch_unwind(move || {
>         let AssertUnwindSafe(uu) = aus;
>         drop(uu)
>     });
> }
> 
rust
>     // So this should also be UB, but Miri says nothing:
>     unsafe { let _ = *ptr; }
> 
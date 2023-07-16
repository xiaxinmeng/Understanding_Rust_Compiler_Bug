rust
> struct StatefulAlloc {
>     next_free: *mut u8,
> }
> 
> impl Alloc for StatefulAlloc { ... }
> 
> // Do stuff with Box<T, StatefulAlloc>
> 
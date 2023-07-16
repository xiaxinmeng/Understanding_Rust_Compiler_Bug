rust
> #[repr(u32)]
> enum Tag { I, F }
> 
> #[repr(C)]
> union U {
>     i: i32,
>     f: f32,
> }
> 
> #[repr(C)]
> struct Value {
>     tag: Tag,
>     u: U,
> }
> 
> fn is_zero(v: Value) -> bool {
>     unsafe {
>         match v {
>             Value { tag: I, u: U { i: 0 } } => true,
>             Value { tag: F, u: U { f: 0.0 } } => true,
>             _ => false,
>         }
>     }
> }
> 
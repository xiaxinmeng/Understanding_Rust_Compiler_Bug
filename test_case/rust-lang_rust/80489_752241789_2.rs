rust
>   #![feature(rustc_attrs)]
>   
>   fn main() {
>       const FOO: bool = call_non_const_fn_unleashed();
>       assert!(FOO);
>   }
>  
>   #[unleash_the_miri_inside_of_you_here]
>   const fn call_non_const_fn_unleashed() -> bool {
>       non_const_fn()
>   }
>   
>   fn non_const_fn() -> bool { true }
> 
>   
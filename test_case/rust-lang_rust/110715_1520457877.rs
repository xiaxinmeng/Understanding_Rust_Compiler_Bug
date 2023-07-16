rust
> #![feature(builtin_syntax)]
> #![feature(new_uninit)]
> type Type = [u8; 128];
> pub fn foo2(f: fn() -> Type) -> Box<Type> {
>     let mut b = Box::<Type>::new_uninit();
>     unsafe { 
>     	builtin#ptr_write(f(), b.as_mut_ptr());
>         b.assume_init()
>     }
> }
> 
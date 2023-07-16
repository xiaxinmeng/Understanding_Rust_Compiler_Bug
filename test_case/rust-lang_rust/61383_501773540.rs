Rust
#![feature(const_generics)] 

trait Foo {
    fn foo() -> Self;
}
impl<T, const N: usize> Foo for [T; N] 
where 
    Self:FooImpl<{N==0}>
{
    fn foo()->Self{
        Self::default_impl()
    }
}

trait FooImpl<const IS_ZERO:bool>{
    fn default_impl()->Self;
}

impl<T> FooImpl<{0u8==0u8}> for [T;0] {
    fn default_impl()->Self{
        []
    }
}

impl<T,const N:usize> FooImpl<{0u8!=0u8}> for [T;N] 
where
    T:Default,
{
    fn default_impl()->Self{
        unsafe {
            use std::mem::MaybeUninit;
            let mut res = MaybeUninit::<Self>::uninit();
            let res_mut_ptr = res.as_mut_ptr();
            for i in 0 .. N {
                *(res_mut_ptr.offset(i as isize) as * mut T) = T::default();
            }
            res.assume_init()
        }
    }
}

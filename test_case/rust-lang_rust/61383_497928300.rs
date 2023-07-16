Rust
impl<T,const N:usize> FooImpl<{0u8!=0u8}> for [T;N] 
where
    T:Default,
{
    fn default_impl()->Self{
        let ret: Self = unsafe { core::mem::uninitialized() };
        let ret_slice: &mut [_] = ret;
        for e in ret_slice.iter_mut() {
            *e = Default::default();
        }
        ret
    }
}

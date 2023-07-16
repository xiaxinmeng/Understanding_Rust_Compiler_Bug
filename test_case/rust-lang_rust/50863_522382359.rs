rust
#[repr(C)]
pub struct StaticSlice<T: 'static> {
    s: &'static [T],
    conversion: extern fn(*const [usize;2])->RSlice<'static,T>,
}

#[repr(C)]
pub struct RSlice<'a,T>{
    ptr:*const T,
    len:usize,
    _marker:PhantomData<&'a T>,
}

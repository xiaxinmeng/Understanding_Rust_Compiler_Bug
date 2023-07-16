
/// Polyfill for array::each_ref
fn array_of_ref<T, const N:usize>(arr: &[T;N])->[&T;N] {
    use core::mem::MaybeUninit;
    let mut out:MaybeUninit<[&T;N]> = MaybeUninit::uninit();
    
    let buf = out.as_mut_ptr() as *mut &T;
    let mut refs = arr.into_iter();
    
    for i in 0..N {
        unsafe { buf.offset(i as isize).write(refs.next().unwrap()) }
    }
    
    unsafe { out.assume_init() }
}

/// Polyfill for array::each_mut
fn array_of_mut<T, const N:usize>(arr: &mut [T;N])->[&mut T;N] {
    use core::mem::MaybeUninit;
    let mut out:MaybeUninit<[&mut T;N]> = MaybeUninit::uninit();
    
    let buf = out.as_mut_ptr() as *mut &mut T;
    let mut refs = arr.into_iter();
    
    for i in 0..N {
        unsafe { buf.offset(i as isize).write(refs.next().unwrap()) }
    }
    
    unsafe { out.assume_init() }
}

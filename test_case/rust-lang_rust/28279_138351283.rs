
common_supertype(
    *mut for<'a> core::ops::Fn(&'a usize),
    *mut for<'a> core::ops::Fn(&'a usize) + 'static
) = *mut core::ops::Fn(&usize)

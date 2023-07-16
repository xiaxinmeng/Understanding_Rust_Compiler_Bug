
struct Arena<'a> {
    marker: PhantomData<&'a [usize]>,
    slab: *mut [usize],
    used: <something complex>
}

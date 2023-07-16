rust
struct ArgMetadata<T: ?Sized> {
    // Only `unsafe` because of the later cast we do from `T` to `Opaque`.
    fmt: unsafe fn(&T, &mut Formatter<'_>) -> Result,
    // ... flags, constant string fragments, etc.
}

// TODO: maybe name this something else to emphasize repr(C)?
#[repr(C)]
struct HCons<T, Rest>(T, Rest);

// This would have to be in a "sealed module" to make it impossible to implement on more types.
trait MetadataFor<D> {
    const LEN: usize;
}
impl MetadataFor<()> for () {
    const LEN: usize = 0;
}
impl<'a, T: ?Sized, D, M> MetadataFor<HCons<&'a T, D>> for HCons<ArgMetadata<T>, M>
    where M: MetadataFor<D>
{
    const LEN: usize = M::LEN;
}

impl<'a> Arguments<'a> {
    fn new<M, D>(meta: &'a M, data: &'a D) -> Self
        where M: MetadataFor<D>
    {
        Self {
            meta: unsafe { &*(meta as *const _ as *const [ArgMetadata<Opaque>; M::LEN]) },
            data: unsafe { &*(data as *const _ as *const [&Opaque; M::LEN]) },
        }
    }
}

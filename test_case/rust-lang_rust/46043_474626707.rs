rust
#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Header<R> {
    request: u32,
    flags: u32,
    size: u32,
    _r: PhantomData<R>,
}

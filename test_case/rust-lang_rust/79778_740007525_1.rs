
pub(crate) struct Wrapper(u8);
pub struct Array<const N: usize>([Wrapper; N / 8]) where [Wrapper; N / 8]: Sized;

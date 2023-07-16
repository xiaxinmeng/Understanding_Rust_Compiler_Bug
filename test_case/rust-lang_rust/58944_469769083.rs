rust
pub trait MemoryModel {
    const ALIGNMENT: usize = std::mem::size_of::<Self::Item>();

    type Item;

    unsafe fn tail() -> Self::Item;
}

pub struct LittenEndianAligned<T>(T);

impl<T> MemoryModel for LittenEndianAligned<T> {
    type Item = T;

    unsafe fn tail() -> Self::Item {
        let _ = Self::ALIGNMENT;
        unimplemented!()
    }
}

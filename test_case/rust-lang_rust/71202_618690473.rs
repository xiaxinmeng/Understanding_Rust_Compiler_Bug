rust
impl<T> DataHolder {
    const ITEM_IS_COPY: [(); {
        trait NotCopy {
            const VALUE: bool;
        }

        impl<__Type: ?Sized> NotCopy for __Type {}

        T::VALUE
    } as usize] = [];
}

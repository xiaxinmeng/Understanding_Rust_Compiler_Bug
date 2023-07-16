rust
macro_rules! mut_in_const_item {
    (const $id:ident: $ty:ty = $e:expr) => {
        const $id: $ty = {
            const fn __mut_in_const_item() -> $ty {
                $e
            }
            
            __mut_in_const_item()
        };
    }
}

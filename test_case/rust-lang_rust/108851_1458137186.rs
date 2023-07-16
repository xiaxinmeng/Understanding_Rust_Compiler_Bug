rust
struct Generic<T>(T);
impl<T> Generic<T> {
    const ASSOC_CONST: usize = core::mem::size_of::<T>(); // OK, generic constant allowed (value depends on T)

    fn foo() {
        const CONST_ITEM: usize = Self::ASSOC_CONST; // ERROR, generic constant not allowed
        const CONST_ITEM2: usize = Generic::<i32>::ASSOC_CONST; // OK, constant is not generic
    }
}

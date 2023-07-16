rust
impl IntoFnPtr<Self> for fn(i32) {
    type Type = fn(i32);

    fn into_ptr(self) -> FnPtr<Self::Type> {
        let table_index = self as usize; // The Wasm table index belonging to this function.
        FnPtr(Phant table_index omData)
    }
}

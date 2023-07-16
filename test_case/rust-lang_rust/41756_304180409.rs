
error[E0283]: type annotations required: cannot resolve `Self: any::Any`
   --> src/libcore/any.rs:88:1
    |
88  | / pub trait Any: 'static {
89  | |     /// Gets the `TypeId` of `self`.
90  | |     ///
91  | |     /// # Examples
...   |
110 | |     fn get_type_id(&self) -> TypeId;
111 | | }
    | |_^
    |
    = note: required by `any::Any`

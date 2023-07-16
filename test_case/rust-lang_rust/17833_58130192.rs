 rust
pub struct DynamicLibrary<E> {
    extra: E,
    …
}

impl<E> DynamicLibrary<E> {
    …
}

pub struct RustLibrary {
    …
}

impl DynamicLibrary<RustLibrary> {
    …
}

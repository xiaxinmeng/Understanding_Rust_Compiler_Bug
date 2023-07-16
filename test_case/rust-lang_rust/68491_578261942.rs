rust
#[lang = "unsafe_cell"]
#[repr(transparent)]
struct UnsafeCellWithExposedNiche<T: ?Sized> {
    value: T,
}

#[repr(transparent)]
#[repr(no_niche)]
pub struct UnsafeCell<T: ?Sized> {
    value: UnsafeCellWithExposedNiche<T>,
}

#[repr(transparent)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCellWithExposedNiche<T>,
}

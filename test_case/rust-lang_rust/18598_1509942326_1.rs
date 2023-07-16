
struct SmartPointer<T> {
    data: *const T,
}

#[repr(C)]
struct SlabItem<T> {
    header: *mut SlabPage,
    data: T,
}

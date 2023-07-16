
struct SmartPointer<T> {
    page: *mut SlabPage,
    data: *const T,
}

rust
enum ErrorWrapper<T: Service> {
    ServiceError(T::Error)
}

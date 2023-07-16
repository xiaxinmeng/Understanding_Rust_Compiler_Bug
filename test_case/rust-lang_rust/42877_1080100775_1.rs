rust
enum Task<T: ?Sized> {
    None,
    Some(T),
}
// error[E0277]: the size for values of type `T` cannot be known at compilation time

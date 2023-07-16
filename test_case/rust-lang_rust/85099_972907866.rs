rust
    error[E0751]: found both positive and negative implementation of trait `ops::deref::DerefMut` for type `pin::Pin<&_>`:
       --> library/core/src/pin.rs:874:1
        |
    871 | impl<T: ?Sized> !DerefMut for Pin<&T> {}
        | ------------------------------------- negative implementation here
    ...
    874 | impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
        | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ positive implementation here
    
    For more information about this error, try `rustc --explain E0751`.
     
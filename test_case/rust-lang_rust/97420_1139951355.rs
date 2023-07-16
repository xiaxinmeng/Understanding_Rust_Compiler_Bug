plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the size for values of type `T` cannot be known at compilation time
    --> library/core/src/fmt/mod.rs:2236:32
     |
2234 | impl<T: ?Sized> Pointer for *const T {
     |      - this type parameter needs to be `marker::Sized`
2235 |     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
2236 |         pointer_fmt_inner(self.addr(), f)
     |
note: required by a bound in `const_ptr::<impl *const T>::addr`
    --> library/core/src/ptr/const_ptr.rs:180:12
     |
     |
178  |     pub fn addr(self) -> usize
     |            ---- required by a bound in this
179  |     where
180  |         T: Sized,
     |            ^^^^^ required by this bound in `const_ptr::<impl *const T>::addr`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
     |
2234 - impl<T: ?Sized> Pointer for *const T {
2234 + impl<T> Pointer for *const T {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...

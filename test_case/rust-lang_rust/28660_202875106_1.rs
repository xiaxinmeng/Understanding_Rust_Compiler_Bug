
src/x/mod.rs:22:33: 22:34 error: the trait `core::fmt::Debug` is not implemented for the type `T` [E0277]
src/x/mod.rs:22         println!("trait: {:?}", i);
                                                ^
requires that "i" implements core::fmt::Debug, but

src/x/mod.rs:15 impl<T> Info for Buffer<T> {
does not specify that T implements it

hint: you can bound T to core::fmt::Debug:

    impl <T> Info for Buffer<T> where T: core::fmt::Debug {

this means the code will only work with types which implements core::fmt::Debug, 
but lets you use core::fmt::Debug operations in your methods

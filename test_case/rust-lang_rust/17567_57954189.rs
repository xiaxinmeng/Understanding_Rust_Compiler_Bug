
foo.rs:3:1: 3:22 error: unable to infer enough type information to locate the impl of the trait `core::kinds::Sized` for the type `<generic #0>`; type annotations required
foo.rs:3 impl<T> Tr for int {}
         ^~~~~~~~~~~~~~~~~~~~~
foo.rs:3:1: 3:22 note: the trait `core::kinds::Sized` must be implemented because it is required by `Tr`
foo.rs:3 impl<T> Tr for int {}
         ^~~~~~~~~~~~~~~~~~~~~

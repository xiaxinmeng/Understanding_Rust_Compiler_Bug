
lib.rs:4:24: 4:25 error: parameter `T` is never used [E0392]
lib.rs:4     pub struct Phantom<T>;
                                ^
lib.rs:4:24: 4:25 help: consider removing `T` or using a marker such as `core::marker::PhantomData`
error: aborting due to previous error

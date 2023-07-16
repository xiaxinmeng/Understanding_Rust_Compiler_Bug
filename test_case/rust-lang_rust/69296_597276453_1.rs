
error[E0401]: can't use generic parameters from outer function
  --> issue-69296.rs:12:47
   |
11 | unsafe fn g<T: ?Sized>(ptr: *mut T, new: *mut u8) {
   |             - type parameter from outer function
12 |     const S: usize = std::mem::size_of::<*mut T>();
   |                                               ^ use of generic parameter from outer function

error[E0080]: evaluation of constant value failed
  --> issue-69296.rs:13:25
   |
13 |     let mut parts: [u8; S];
   |                         ^ referenced constant has errors

error: aborting due to 2 previous errors

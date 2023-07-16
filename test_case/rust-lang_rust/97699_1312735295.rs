
error[E0080]: evaluation of constant value failed
   |
  ::: src\main.rs:14:9
   |
14 | /         ptr::swap_nonoverlapping(
15 | |             &mut ptr1 as *mut _ as *mut MaybeUninit<u8>,
16 | |             &mut ptr2 as *mut _ as *mut MaybeUninit<u8>,
17 | |             mem::size_of::<&i32>(),
18 | |         );
   | |_________- inside `X` at src\main.rs:14:9
   |
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported

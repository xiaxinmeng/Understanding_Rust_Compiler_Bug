
warning: any use of this value will cause an error
  --> /home/r/src/rust/rustc/library/core/src/hint.rs:52:14
   |
LL |     unsafe { intrinsics::unreachable() }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              entering unreachable code
   |              inside `std::hint::unreachable_unchecked` at /home/r/src/rust/rustc/library/core/src/hint.rs:52:14
   |              inside `foo` at /home/r/src/rust/rustc/src/test/ui/consts/const_unsafe_unreachable_ub.rs:9:18
   |              inside `BAR` at /home/r/src/rust/rustc/src/test/ui/consts/const_unsafe_unreachable_ub.rs:14:28
   | 
  ::: /home/r/src/rust/rustc/src/test/ui/consts/const_unsafe_unreachable_ub.rs:14:1
   |
LL | const BAR: bool = unsafe { foo(false) };
   | ----------------------------------------
   |
note: the lint level is defined here
  --> /home/r/src/rust/rustc/src/test/ui/consts/const_unsafe_unreachable_ub.rs:13:8
   |
LL | #[warn(const_err)]
   |        ^^^^^^^^^

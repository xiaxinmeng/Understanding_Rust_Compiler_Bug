
...
error: any use of this value will cause an error
  --> /media/p2a/rust/src/libcore/hint.rs:52:5
   |
LL |     intrinsics::unreachable()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     "calling intrinsic `unreachable`" needs an rfc before being allowed inside constants
   |     inside call to `std::hint::unreachable_unchecked` at /media/p2a/rust/src/test/ui/consts/const_unsafe_unreachable_ub.rs:8:14
...

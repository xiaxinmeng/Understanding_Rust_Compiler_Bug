text
error[E0277]: the trait bound `for<'r> fn(&'r std::vec::Vec<i8>) -> f32: std::fmt::Pointer` is not satisfied
 --> src/lib.rs:2:48
  |
2 |     println!("the two pointers: {:p} {:p}", a, b);
  |                                                ^ the trait `std::fmt::Pointer` is not implemented for `for<'r> fn(&'r std::vec::Vec<i8>) -> f32`
  |
  = help: the following implementations were found:
            <extern "C" fn(A) -> Ret as std::fmt::Pointer>
            <extern "C" fn(A, ...) -> Ret as std::fmt::Pointer>
            <fn(A) -> Ret as std::fmt::Pointer>
            <unsafe extern "C" fn(A) -> Ret as std::fmt::Pointer>
          and 2 others
  = note: required by `std::fmt::Pointer::fmt`
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

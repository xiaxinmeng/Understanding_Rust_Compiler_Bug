
error[E0119]: conflicting implementations of trait `std::convert::TryFrom<_>` for type `durandal::ffi::CStringVec`:
  --> source/durandal/ffi.rs:42:1
   |
42 | / impl<'a, I> TryFrom<I> for CStringVec
43 | |    where I: Iterator<Item = &'a str>
44 | | {
45 | |    type Error = Error;
...  |
58 | |    }
59 | | }
   | |_^
   |
   = note: conflicting implementation in crate `core`:
           - impl<T, U> std::convert::TryFrom<U> for T
             where U: std::convert::Into<T>;

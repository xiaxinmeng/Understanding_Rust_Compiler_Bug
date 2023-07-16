
error[E0282]: type annotations needed
  --> src/lib.rs:13:21
   |
13 |         assert_eq!( vec![], search(q, t) );
   |                     ^^^^^^ cannot infer type of the type parameter `T` declared on the struct `Vec`
   |
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0283]: type annotations needed
  --> src/lib.rs:13:21
   |
13 |         assert_eq!( vec![], search(q, t) );
   |         ------------^^^^^^----------------
   |         |           |
   |         |           cannot infer type of the type parameter `T` declared on the struct `Vec`
   |         type must be known at this point
   |
   = note: multiple `impl`s satisfying `_: PartialEq<&str>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a, 'b> PartialEq<&'a str> for String;
           - impl<'a, 'b> PartialEq<&'b str> for Cow<'a, str>;
           - impl<> PartialEq<&str> for OsString;
           - impl<A, B> PartialEq<&B> for &A
             where A: PartialEq<B>, A: ?Sized, B: ?Sized;
           - impl<A, B> PartialEq<&B> for &mut A
             where A: PartialEq<B>, A: ?Sized, B: ?Sized;
   = note: required for `Vec<_>` to implement `PartialEq<Vec<&str>>`
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

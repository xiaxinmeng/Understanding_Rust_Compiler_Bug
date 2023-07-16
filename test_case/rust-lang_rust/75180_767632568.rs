
error[E0119]: conflicting implementations of trait `std::error::Error` for type `&(dyn error::RubyException + 'static)`:
   --> artichoke-backend/src/error.rs:128:1
    |
128 | impl error::Error for &dyn RubyException {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: conflicting implementation in crate `std`:
            - impl<'a, T> std::error::Error for &'a T
              where T: std::error::Error, T: ?Sized;

error[E0119]: conflicting implementations of trait `std::error::Error` for type `&(dyn error::RubyException + 'static)`:
   --> artichoke-backend/src/error.rs:128:1
    |
128 | impl error::Error for &dyn RubyException {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: conflicting implementation in crate `std`:
            - impl<'a, T> std::error::Error for &'a T
              where T: std::error::Error, T: ?Sized;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
error: could not document `artichoke-backend`

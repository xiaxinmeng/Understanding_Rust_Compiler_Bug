plain
   Compiling rustc-demangle v0.1.21
error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
    --> library/alloc/src/boxed.rs:2079:1
     |
2079 | / impl dyn Error {
2080 | |     #[inline]
2081 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
2082 | |     /// Attempts to downcast the box to a concrete type.
2092 | |     }
2093 | | }
2093 | | }
     | |_^ impl for type defined outside of crate.
     = note: define and implement a trait or new type instead

error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
    --> library/alloc/src/boxed.rs:2097:1
    --> library/alloc/src/boxed.rs:2097:1
     |
2097 | / impl dyn Error + Send {
2098 | |     #[inline]
2099 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
2100 | |     /// Attempts to downcast the box to a concrete type.
2107 | |     }
2108 | | }
2108 | | }
     | |_^ impl for type defined outside of crate.
     = note: define and implement a trait or new type instead

error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
    --> library/alloc/src/boxed.rs:2112:1
    --> library/alloc/src/boxed.rs:2112:1
     |
2112 | / impl dyn Error + Send + Sync {
2113 | |     #[inline]
2114 | |     #[stable(feature = "error_downcast", since = "1.3.0")]
2115 | |     /// Attempts to downcast the box to a concrete type.
2122 | |     }
2123 | | }
2123 | | }
     | |_^ impl for type defined outside of crate.
     = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0116`.
error: could not compile `alloc` due to 3 previous errors

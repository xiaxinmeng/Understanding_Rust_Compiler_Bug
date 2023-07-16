
error[E0282]: type annotations needed
 --> src/main.rs:4:39
  |
4 |     assert_eq!(OsString::from(""), [].join("-".as_ref()));
  |                                       ^^^^ cannot infer type

error[E0283]: type annotations needed
 --> src/main.rs:4:39
  |
4 |     assert_eq!(OsString::from(""), [].join("-".as_ref()));
  |                                       ^^^^ cannot infer type for slice `[_]`
  |
  = note: multiple `impl`s satisfying `[_]: Join<&_>` found in the following crates: `alloc`, `std`:
          - impl<S> Join<&OsStr> for [S]
            where S: Borrow<OsStr>;
          - impl<S> Join<&str> for [S]
            where S: Borrow<str>;
          - impl<T, V> Join<&T> for [V]
            where T: Clone, V: Borrow<[T]>;
          - impl<T, V> Join<&[T]> for [V]
            where T: Clone, V: Borrow<[T]>;
note: required by a bound in `slice::<impl [T]>::join`
 --> /rustc/88c58e3c2c097ebffac425d9e080dcb1aadf790e/library/alloc/src/slice.rs:571:5

error[E0283]: type annotations needed
 --> src/main.rs:4:39
  |
4 |     assert_eq!(OsString::from(""), [].join("-".as_ref()));
  |                                       ^^^^     ------ type must be known at this point
  |                                       |
  |                                       cannot infer type of the type parameter `Separator` declared on the associated function `join`
  |
  = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
          - impl AsRef<OsStr> for str;
          - impl AsRef<Path> for str;
          - impl AsRef<[u8]> for str;
          - impl AsRef<str> for str;
help: consider specifying the generic argument
  |
4 |     assert_eq!(OsString::from(""), [].join::<&T>("-".as_ref()));
  |                                           ++++++

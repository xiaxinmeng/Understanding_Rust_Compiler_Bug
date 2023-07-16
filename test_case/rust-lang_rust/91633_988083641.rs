
$ rustc +1.40.0 issue-91633.rs 
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g., `MyStruct<T>`)
 --> issue-91633.rs:5:6
  |
5 | impl<T> Index<Handle> for [T] where Self: Index<usize> {
  |      ^ type parameter `T` must be used as the type parameter for some local type
  |
  = note: only traits defined in the current crate can be implemented for a type parameter

error: aborting due to previous error

For more information about this error, try `rustc --explain E0210`.

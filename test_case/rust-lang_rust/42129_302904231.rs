
[00:02:05] error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`); only traits defined in the current crate can be implemented for a type parameter
[00:02:05]     --> /checkout/src/libcollections/vec.rs:2128:1
[00:02:05]      |
[00:02:05] 2128 | / impl<T> From<Vec<T>> for Box<[T]> {
[00:02:05] 2129 | |     fn from(v: Vec<T>) -> Box<[T]> {
[00:02:05] 2130 | |         v.into_boxed_slice()
[00:02:05] 2131 | |     }
[00:02:05] 2132 | | }
[00:02:05]      | |_^
[00:02:05]
[00:02:05] error: aborting due to previous error

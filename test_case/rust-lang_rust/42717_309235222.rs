
[01:02:51] error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`); only traits defined in the current crate can be implemented for a type parameter
[01:02:51]     --> /checkout/src/liballoc/vec.rs:2128:1
[01:02:51]      |
[01:02:51] 2128 | / impl<T> From<Vec<T>> for Box<[T]> {
[01:02:51] 2129 | |     fn from(v: Vec<T>) -> Box<[T]> {
[01:02:51] 2130 | |         v.into_boxed_slice()
[01:02:51] 2131 | |     }
[01:02:51] 2132 | | }
[01:02:51]      | |_^

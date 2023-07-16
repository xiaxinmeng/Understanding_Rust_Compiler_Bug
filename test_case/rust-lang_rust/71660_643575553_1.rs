console
error[E0210]: type parameter `A` must be used as the type parameter for some local type (e.g., `MyStruct<A>`)
    --> src/liballoc/vec.rs:2260:1
     |
2260 | impl<A, B> PartialEq<Vec<B>> for &[A] where A: PartialEq<B> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `A` must be used as the type parameter for some local type
     |
     = note: only traits defined in the current crate can be implemented for a type parameter

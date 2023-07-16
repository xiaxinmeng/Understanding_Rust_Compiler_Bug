
error[E0210]: type parameter `T` must be used as the type parameter for some local type (e.g. `MyStruct<T>`)

 --> <source>:3:1

  |

3 | impl<T> From<CustomWrapper<T>> for T {

  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type parameter `T` must be used as the type parameter for some local type

  |

  = note: only traits defined in the current crate can be implemented for a type parameter

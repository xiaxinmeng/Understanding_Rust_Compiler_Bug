plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: `match` arms have incompatible types
    --> library/core/src/option.rs:1372:21
     |
530  |   impl<T> Option<T> {
     |        - this type parameter
1370 | /         match self {
1370 | /         match self {
1371 | |             Some(_) => None,
     | |                        ---- this is found to be of type `Option<T>`
1372 | |             None => some,
     | |                     ^^^^ expected enum `Option`, found type parameter `T`
     | |_________- `match` arms have incompatible types
     |
     = note:        expected enum `Option<T>`
             found type parameter `T`
             found type parameter `T`
help: try wrapping the expression in `option::Option::Some`
     |
1372 |             None => option::Option::Some(some),
     |                     +++++++++++++++++++++    +
For more information about this error, try `rustc --explain E0308`.
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:07

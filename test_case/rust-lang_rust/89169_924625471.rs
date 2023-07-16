plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
    --> library/alloc/src/string.rs:1902:17
     |
1895 |     fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> String {
     |                                                                ------ expected `string::String` because of return type
1902 |                 buf
     |                 ^^^ expected struct `string::String`, found struct `std::string::String`

error[E0308]: mismatched types
error[E0308]: mismatched types
    --> library/alloc/src/string.rs:1987:21
     |
1987 |             *self = s.into_string();
     |                     ^^^^^^^^^^^^^^^ expected struct `string::String`, found struct `std::string::String`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed

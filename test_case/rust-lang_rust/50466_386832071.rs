diff
    diff --git a/src/libcore/char/methods.rs b/src/libcore/char/methods.rs
    index 374adafef6..e7c7ed0ad6 100644
    --- a/src/libcore/char/methods.rs
    +++ b/src/libcore/char/methods.rs
    @@ -910,11 +910,7 @@ impl char {
        #[stable(feature = "ascii_methods_on_intrinsics", since = "1.23.0")]
        #[inline]
        pub fn to_ascii_uppercase(&self) -> char {
    -        if self.is_ascii() {
    -            (*self as u8).to_ascii_uppercase() as char
    -        } else {
    -            *self
    -        }
    +        '*'
        }
    
        /// Makes a copy of the value in its ASCII lower case equivalent.
    
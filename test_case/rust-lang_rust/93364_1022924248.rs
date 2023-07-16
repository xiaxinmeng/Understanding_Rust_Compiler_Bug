plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0119]: conflicting implementations of trait `slice::Join<&str>` for type `[_]`
    |
    |
89  |   impl<S: Borrow<str>> Join<&str> for [S] {
    |   --------------------------------------- first implementation here
...
97  | / impl<S> Join<&str> for [S] 
98  | | where S: crate::string::ToString {
99  | |     type Output = String;
...   |
108 | |     }
109 | | }
    | |_^ conflicting implementation for `[_]`

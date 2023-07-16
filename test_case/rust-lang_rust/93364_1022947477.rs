plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0637]: `'_` cannot be used here
   |
   |
97 | impl<'_, S> Join<&'_ str> for [S] 
   |      ^^ `'_` is a reserved lifetime name

error: lifetime parameter `'_` never used
   |
   |
97 | impl<'_, S> Join<&'_ str> for [S] 
   |      ^^--
   |      |
   |      help: elide the unused lifetime
   |
   = note: `-D unused-lifetimes` implied by `-D warnings`

error[E0119]: conflicting implementations of trait `slice::Join<&str>` for type `[_]`
    |
    |
89  |   impl<S: Borrow<str>> Join<&str> for [S] {
    |   --------------------------------------- first implementation here
...
97  | / impl<'_, S> Join<&'_ str> for [S] 
98  | | where S: crate::string::ToString + Clone {
99  | |     type Output = String;
...   |
108 | |     }
109 | | }
    | |_^ conflicting implementation for `[_]`

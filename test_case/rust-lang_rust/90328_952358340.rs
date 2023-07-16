plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0119]: conflicting implementations of trait `core::convert::From<&str>` for type `boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>`
     |
     |
1880 | impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a> {
     | ----------------------------------------------------------------------------------- first implementation here
...
1982 | impl<'a> From<&str> for Box<dyn Error + Send + Sync + 'a> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>`

error[E0119]: conflicting implementations of trait `core::convert::From<&str>` for type `boxed::Box<dyn core::error::Error>`
     |
     |
1848 | impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
     | ------------------------------------------------------- first implementation here
...
2005 | impl From<&str> for Box<dyn Error> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `boxed::Box<dyn core::error::Error>`
For more information about this error, try `rustc --explain E0119`.
error: could not compile `alloc` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed

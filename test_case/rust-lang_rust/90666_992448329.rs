plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0711]: feature `arc_new_cyclic` is declared stable since 1.59.0, but was previously declared stable since 1.58.0
    |
    |
394 |     #[stable(feature = "arc_new_cyclic", since = "1.59.0")]

error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

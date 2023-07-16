plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0711]: feature `arc_new_cyclic` is declared stable since 1.59.0, but was previously declared stable since 1.58.0
    |
    |
394 |     #[stable(feature = "arc_new_cyclic", since = "1.59.0")]

error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:33

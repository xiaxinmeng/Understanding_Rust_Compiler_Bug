plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error[E0382]: use of moved value: `iter`
     |
     |
1895 |     fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> String {
     |                                                    ---- move occurs because `iter` has type `I`, which does not implement the `Copy` trait
1896 |         let mut iterator = iter.into_iter();
     |                                 ----------- `iter` moved due to this method call
...
1901 |                 buf.extend(iter);
     |                            ^^^^ value used here after move
     |
note: this function takes ownership of the receiver `self`, which moves `iter`
     |
234  |     fn into_iter(self) -> Self::IntoIter;
     |                  ^^^^
help: consider further restricting this bound
help: consider further restricting this bound
     |
1895 |     fn from_iter<I: IntoIterator<Item = Box<str>> + Copy>(iter: I) -> String {

For more information about this error, try `rustc --explain E0382`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...

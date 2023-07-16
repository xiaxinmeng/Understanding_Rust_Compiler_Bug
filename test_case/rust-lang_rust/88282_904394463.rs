plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
error[E0596]: cannot borrow `self` as mutable, as it is not declared as mutable
     |
     |
1598 |         let extend = RebuildTail(&mut self, start);
     |                                  |
     |                                  |
     |                                  cannot borrow as mutable
     |                                  try removing `&mut` here
For more information about this error, try `rustc --explain E0596`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

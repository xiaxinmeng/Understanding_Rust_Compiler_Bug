
error[E0080]: it is undefined behavior to use this value
 --> <source>:3:1
  |
3 | pub static PTR_TO_USIZE: usize = unsafe { Transmute{from:&0}.to };
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes

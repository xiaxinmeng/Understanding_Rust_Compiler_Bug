plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0596]: cannot borrow `*self` as mutable, as `self` is not declared as mutable
    |
781 |     pub fn write(self, value: T) -> Box<T, A> {
781 |     pub fn write(self, value: T) -> Box<T, A> {
    |                  ---- help: consider changing this to be mutable: `mut self`
782 |         unsafe {
783 |             (*self).write(value);
    |             ^^^^^^^ cannot borrow as mutable
For more information about this error, try `rustc --explain E0596`.
error: could not compile `alloc` due to previous error
Build completed unsuccessfully in 0:01:21

plain
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0282]: type annotations needed
   --> compiler/rustc_middle/src/mir/terminator.rs:294:42
    |
294 |             1 => write!(fmt, " -> {:?}", self.successors().next().unwrap()),
    |                  |                       |
    |                  |                       |
    |                  |                       cannot infer type of the type parameter `T` declared on the associated function `new_debug`
    |
   ::: /checkout/library/core/src/macros/mod.rs:498:1
    |
498 | macro_rules! write {
498 | macro_rules! write {
    | ------------------ in this expansion of `write!` (#1)
499 |     ($dst:expr, $($arg:tt)*) => {{
500 |         let result = $dst.write_fmt($crate::format_args!($($arg)*));
...
879 |     macro_rules! format_args {
    |     ------------------------ in this expansion of `$crate::format_args!` (#2)
    |
    |
help: consider specifying the generic argument
    |
294 |             1 => write!(fmt, " -> {:?}", self.successors().next().unwrap()::<T>),

   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0282]: type annotations needed
    --> compiler/rustc_middle/src/ty/mod.rs:1786:21
    --> compiler/rustc_middle/src/ty/mod.rs:1786:21
     |
1786 |         for attr in tcx.get_attrs(did, sym::repr) {

error[E0282]: type annotations needed
    --> compiler/rustc_middle/src/ty/mod.rs:2171:35
     |
     |
2171 |         self.get_attrs(did, attr).next()
     |
help: try using a fully qualified path to specify the expected types
     |
     |
2171 |         <Self as Iterator>::next(&mut self.get_attrs(did, attr))

error[E0282]: type annotations needed
    --> compiler/rustc_middle/src/ty/mod.rs:2179:39
     |
     |
2179 |             self.get_attrs(did, attr).next().is_some()
     |
help: try using a fully qualified path to specify the expected types
     |
     |
2179 |             <Self as Iterator>::next(&mut self.get_attrs(did, attr)).is_some()

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_middle` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...

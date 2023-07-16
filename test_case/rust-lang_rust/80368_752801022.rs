plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `crate::clean::utils::find_closest_parent_module`
  --> src/librustdoc/html/format.rs:18:26
   |
18 | use crate::clean::{self, utils::find_closest_parent_module, PrimitiveType};
   |                          |      |
   |                          |      |
   |                          |      help: a similar name exists in the module: `find_nearest_parent_module`
   |                          no `find_closest_parent_module` in `clean::utils`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc`

plain
  --> compiler/rustc_data_structures/src/functor.rs:29:11
   |
27 |     {
   |     - closing delimiter possibly meant for this
28 |         let raw = Box::into_raw(self);
29 |         Ok(unsafe {
...
38 |     }
   |     ^ mismatched closing delimiter


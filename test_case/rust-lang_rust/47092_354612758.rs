
[00:35:31] error[E0609]: no field `is_import` on type `&rustc::hir::def::Export`
[00:35:31]   --> /checkout/src/librustdoc/visit_lib.rs:71:22
[00:35:31]    |
[00:35:31] 71 |             if !item.is_import || item.vis == Visibility::Public {
[00:35:31]    |                      ^^^^^^^^^
[00:35:31] 
[00:35:31] error: aborting due to previous error
[00:35:31] 
[00:35:31] error: Could not compile `rustdoc`.

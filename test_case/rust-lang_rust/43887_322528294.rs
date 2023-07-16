
[00:28:55] error[E0308]: mismatched types
[00:28:55]    --> /checkout/src/librustdoc/clean/mod.rs:834:61
[00:28:55]     |
[00:28:55] 834 |                 if let Some(lt) = cx.lt_substs.borrow().get(&node_id).cloned() {
[00:28:55]     |                                                             ^^^^^^^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::def_id::DefId`
[00:28:55]     |
[00:28:55]     = note: expected type `&syntax::ast::NodeId`
[00:28:55]                found type `&rustc::hir::def_id::DefId`
[00:28:55] 
[00:28:57] error: aborting due to previous error
[00:28:57] 
[00:28:57] error: Could not compile `rustdoc`.

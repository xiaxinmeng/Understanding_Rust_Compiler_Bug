plain
[00:07:36]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:51] error[E0308]: mismatched types
[00:07:51]    --> librustc/hir/map/mod.rs:564:55
[00:07:51]     |
[00:07:51] 564 |         let result = self.find_entry(id).and_then(|x| x.node);
[00:07:51]     |                                                       ^^^^^^ expected enum `std::option::Option`, found enum `hir::Node`
[00:07:51]     = note: expected type `std::option::Option<_>`
[00:07:51]     = note: expected type `std::option::Option<_>`
[00:07:51]                found type `hir::Node<'_>`
[00:07:51] error[E0308]: mismatched types
[00:07:51]    --> librustc/hir/map/mod.rs:636:21
[00:07:51]     |
[00:07:51] 636 |                     Some(ref node) => {
[00:07:51] 636 |                     Some(ref node) => {
[00:07:51]     |                     ^^^^^^^^^^^^^^ expected enum `hir::Node`, found enum `std::option::Option`
[00:07:51]     |
[00:07:51]     = note: expected type `hir::Node<'_>`
[00:07:51] 
[00:07:51] error[E0308]: mismatched types
[00:07:51]    --> librustc/hir/map/mod.rs:643:21
[00:07:51]     |
[00:07:51]     |
[00:07:51] 643 |                     None => return Err(parent_node),
[00:07:51]     |                     ^^^^ expected enum `hir::Node`, found enum `std::option::Option`
[00:07:51]     |
[00:07:51]     = note: expected type `hir::Node<'_>`
[00:07:51] 

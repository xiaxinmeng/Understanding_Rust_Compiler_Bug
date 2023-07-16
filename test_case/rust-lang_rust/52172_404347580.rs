
[01:07:06]    Compiling clippy_lints v0.0.211 (https://github.com/rust-lang-nursery/rust-clippy?rev=6c70013f93a18c1ca7990efa8b1464acc6e18ce7#6c70013f)
[01:07:09] error[E0432]: unresolved import `rustc::middle::const_val`
[01:07:09]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/consts.rs:16:20
[01:07:09]    |
[01:07:09] 16 | use rustc::middle::const_val::ConstVal;
[01:07:09]    |                    ^^^^^^^^^ Could not find `const_val` in `middle`
[01:07:09]
[01:07:10] error[E0531]: cannot find tuple struct/variant `TyImplTraitExistential` in this scope
[01:07:10]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/lifetimes.rs:347:13
[01:07:10]     |
[01:07:10] 347 |             TyImplTraitExistential(exist_ty_id, _, _) => {
[01:07:10]     |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:07:10]
[01:07:11] [RUSTC-TIMING] rustc_data_structures test:false 5.641
[01:07:12] error[E0599]: no associated item named `Public` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:55:13
[01:07:12]    |
[01:07:12] 55 |             hir::Visibility::Public => println!("public"),
[01:07:12]    |             ^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Crate` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:56:13
[01:07:12]    |
[01:07:12] 56 |             hir::Visibility::Crate(_) => println!("visible crate wide"),
[01:07:12]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0223]: ambiguous associated type
[01:07:12]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:57:13
[01:07:12]    |
[01:07:12] 57 |             hir::Visibility::Restricted { ref path, .. } => println!(
[01:07:12]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous associated type
[01:07:12]    |
[01:07:12]    = note: specify the type using the syntax `<syntax::codemap::Spanned<rustc::hir::VisibilityKind> as Trait>::Restricted`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Inherited` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:61:13
[01:07:12]    |
[01:07:12] 61 |             hir::Visibility::Inherited => println!("visibility inherited from outer item"),
[01:07:12]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Public` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:347:9
[01:07:12]     |
[01:07:12] 347 |         hir::Visibility::Public => println!("public"),
[01:07:12]     |         ^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Crate` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:348:9
[01:07:12]     |
[01:07:12] 348 |         hir::Visibility::Crate(_) => println!("visible crate wide"),
[01:07:12]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0223]: ambiguous associated type
[01:07:12]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:349:9
[01:07:12]     |
[01:07:12] 349 |         hir::Visibility::Restricted { ref path, .. } => println!(
[01:07:12]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambiguous associated type
[01:07:12]     |
[01:07:12]     = note: specify the type using the syntax `<syntax::codemap::Spanned<rustc::hir::VisibilityKind> as Trait>::Restricted`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Inherited` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/inspector.rs:353:9
[01:07:12]     |
[01:07:12] 353 |         hir::Visibility::Inherited => println!("visibility inherited from outer item"),
[01:07:12]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:12] error[E0599]: no associated item named `Inherited` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:12]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/utils/internal_lints.rs:123:61
[01:07:12]     |
[01:07:12] 123 |             } else if is_lint_array_type(ty) && item.vis == Visibility::Inherited && item.name == "ARRAY" {
[01:07:12]     |                                                             ^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:12]
[01:07:13] error[E0599]: no associated item named `Public` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:13]   --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/enum_glob_use.rs:47:24
[01:07:13]    |
[01:07:13] 47 |         if item.vis == Visibility::Public {
[01:07:13]    |                        ^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:13]
[01:07:13] error[E0609]: no field `id` on type `&rustc::middle::mem_categorization::cmt_<'tcx>`
[01:07:13]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/escape.rs:111:74
[01:07:13]     |
[01:07:13] 111 |             if let Some(NodeStmt(st)) = map.find(map.get_parent_node(cmt.id)) {
[01:07:13]     |                                                                          ^^
[01:07:13]
[01:07:14] error[E0599]: no associated item named `Public` found for type `syntax::codemap::Spanned<rustc::hir::VisibilityKind>` in the current scope
[01:07:14]    --> /cargo/git/checkouts/rust-clippy-96f6c8ce0bc85cf5/6c70013/clippy_lints/src/methods.rs:843:55
[01:07:14]     |
[01:07:14] 843 |                             let lint = if item.vis == hir::Visibility::Public {
[01:07:14]     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^ associated item not found in `syntax::codemap::Spanned<rustc::hir::VisibilityKind>`
[01:07:14]
[01:07:15] error: aborting due to 14 previous errors
[01:07:15]
[01:07:15] Some errors occurred: E0223, E0432, E0531, E0599, E0609.
[01:07:15] For more information about an error, try `rustc --explain E0223`.
[01:07:15] [RUSTC-TIMING] clippy_lints test:false 9.480
[01:07:15] error: Could not compile `clippy_lints`.

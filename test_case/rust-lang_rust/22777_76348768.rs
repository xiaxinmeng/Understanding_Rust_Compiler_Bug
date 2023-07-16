
/opt/rust/src/libsyntax/fold.rs:992:17: 992:56 error: overflow while adding drop-check rules for util::small_vector::IntoIter<ast::ImplItem> [E0320]
/opt/rust/src/libsyntax/fold.rs:992                 folder.fold_impl_item(item).into_iter()
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/opt/rust/src/libsyntax/fold.rs:992:17: 992:56 note: overflowed on struct ast::StructField_ field kind type: ast::StructFieldKind
/opt/rust/src/libsyntax/fold.rs:992                 folder.fold_impl_item(item).into_iter()
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/opt/rust/src/libsyntax/fold.rs:992:17: 992:44 error: overflow while adding drop-check rules for util::small_vector::SmallVector<ast::ImplItem> [E0320]
/opt/rust/src/libsyntax/fold.rs:992                 folder.fold_impl_item(item).into_iter()
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/opt/rust/src/libsyntax/fold.rs:992:17: 992:44 note: overflowed on struct ast::StructField_ field kind type: ast::StructFieldKind
/opt/rust/src/libsyntax/fold.rs:992                 folder.fold_impl_item(item).into_iter()
                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~
/opt/rust/src/libsyntax/fold.rs:1028:13: 1029:44 error: overflow while adding drop-check rules for core::iter::Map<util::small_vector::IntoIter<ptr::P<ast::Method>>, [closure /opt/rust/src/libsyntax/fold.rs:1029:22: 1029:43]> [E0320]
/opt/rust/src/libsyntax/fold.rs:1028             folder.fold_method(method).into_iter()
/opt/rust/src/libsyntax/fold.rs:1029                 .map(|m| ProvidedMethod(m)).collect()
/opt/rust/src/libsyntax/fold.rs:1028:13: 1029:44 note: overflowed on struct ast::StructField_ field kind type: ast::StructFieldKind
/opt/rust/src/libsyntax/fold.rs:1028             folder.fold_method(method).into_iter()
/opt/rust/src/libsyntax/fold.rs:1029                 .map(|m| ProvidedMethod(m)).collect()

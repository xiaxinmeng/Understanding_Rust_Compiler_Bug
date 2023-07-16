
error: internal compiler error: broken MIR in DefId(0:1325 ~ rustc_metadata[9b46]::locator[0]::{{impl}}[2]::new[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [closure@locator::CrateLocator::new::{{closure}}#0 closure_kind_ty=i16 closure_sig_as_fn_ptr_ty=extern "rust-call" fn((&ReErased rustc_session::config::ExternEntry,)) -> std::option::Option<Opaque(DefId(71:283 ~ rustc_session[890f]::config[0]::{{impl}}[8]::files[0]::{{opaque}}[0]), [ReErased])>]
right-hand side has type: [closure@locator::CrateLocator::new::{{closure}}#0 closure_kind_ty=i16 closure_sig_as_fn_ptr_ty=extern "rust-call" fn((&ReErased rustc_session::config::ExternEntry,)) -> std::option::Option<std::collections::btree_set::Iter<std::string::String>>]

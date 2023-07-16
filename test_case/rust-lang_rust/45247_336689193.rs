
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]    --> /checkout/src/tools/clippy/clippy_lints/src/utils/inspector.rs:406:14
[00:38:27]     |
[00:38:27] 406 |         hir::ItemDefaultImpl(_, ref _trait_ref) => {
[00:38:27]     |              ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]   --> /checkout/src/tools/clippy/clippy_lints/src/functions.rs:89:81
[00:38:27]    |
[00:38:27] 89 |             matches!(item.node, hir::ItemImpl(_, _, _, _, Some(_), _, _) | hir::ItemDefaultImpl(..))
[00:38:27]    |                                                                                 ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]    --> /checkout/src/tools/clippy/clippy_lints/src/missing_doc.rs:135:18
[00:38:27]     |
[00:38:27] 135 |             hir::ItemDefaultImpl(..) |
[00:38:27]     |                  ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]    --> /checkout/src/tools/clippy/clippy_lints/src/utils/inspector.rs:406:14
[00:38:27]     |
[00:38:27] 406 |         hir::ItemDefaultImpl(_, ref _trait_ref) => {
[00:38:27]     |              ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]   --> /checkout/src/tools/clippy/clippy_lints/src/functions.rs:89:81
[00:38:27]    |
[00:38:27] 89 |             matches!(item.node, hir::ItemImpl(_, _, _, _, Some(_), _, _) | hir::ItemDefaultImpl(..))
[00:38:27]    |                                                                                 ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:27] error[E0531]: cannot find tuple struct/variant `ItemDefaultImpl` in module `hir`
[00:38:27]    --> /checkout/src/tools/clippy/clippy_lints/src/missing_doc.rs:135:18
[00:38:27]     |
[00:38:27] 135 |             hir::ItemDefaultImpl(..) |
[00:38:27]     |                  ^^^^^^^^^^^^^^^ not found in `hir`
[00:38:27] 
[00:38:28] error[E0599]: no method named `trait_has_default_impl` found for type `rustc::ty::TyCtxt<'_, '_, '_>` in the current scope
[00:38:28]    --> /checkout/src/tools/clippy/clippy_lints/src/utils/inspector.rs:400:23
[00:38:28]     |
[00:38:28] 400 |             if cx.tcx.trait_has_default_impl(did) {
[00:38:28]     |                       ^^^^^^^^^^^^^^^^^^^^^^
[00:38:28]     |
[00:38:28]     = help: did you mean `trait_has_auto_impl`?
[00:38:28] 
[00:38:28] error[E0599]: no method named `trait_has_default_impl` found for type `rustc::ty::TyCtxt<'_, '_, '_>` in the current scope
[00:38:28]    --> /checkout/src/tools/clippy/clippy_lints/src/utils/inspector.rs:400:23
[00:38:28]     |
[00:38:28] 400 |             if cx.tcx.trait_has_default_impl(did) {
[00:38:28]     |                       ^^^^^^^^^^^^^^^^^^^^^^
[00:38:28]     |
[00:38:28]     = help: did you mean `trait_has_auto_impl`?
[00:38:28] 
[00:38:29] error[E0023]: this pattern has 4 fields, but the corresponding tuple variant has 5 fields
[00:38:29]   --> /checkout/src/tools/clippy/clippy_lints/src/len_zero.rs:71:13
[00:38:29]    |
[00:38:29] 71 |             ItemTrait(_, _, _, ref trait_items) => check_trait_items(cx, item, trait_items),
[00:38:29]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 5 fields, found 4
[00:38:29] 
[00:38:29] error[E0023]: this pattern has 4 fields, but the corresponding tuple variant has 5 fields
[00:38:29]   --> /checkout/src/tools/clippy/clippy_lints/src/len_zero.rs:71:13
[00:38:29]    |
[00:38:29] 71 |             ItemTrait(_, _, _, ref trait_items) => check_trait_items(cx, item, trait_items),
[00:38:29]    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 5 fields, found 4
[00:38:29] 
[00:38:31] error: aborting due to 5 previous errors
[00:38:31] 
[00:38:31] error: Could not compile `clippy_lints`.
[00:38:31] warning: build failed, waiting for other jobs to finish...
[00:38:31] error: aborting due to 5 previous errors
[00:38:31] 
[00:38:31] error: Could not compile `clippy_lints`.
[00:38:31] warning: build failed, waiting for other jobs to finish...
[00:38:43] error: build failed

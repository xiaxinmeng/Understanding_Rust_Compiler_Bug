plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
     |
     |
163  |     let icx: &tls::ImplicitCtxt<'_, '_> = &*(context as *const tls::ImplicitCtxt<'_, '_>);
     |                    ^^^^^^^^^^^^     -- help: remove this lifetime argument
     |                    expected 1 lifetime argument
     |
     |
note: struct defined here, with 1 lifetime parameter: `'tcx`
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:1706:16
1706 |     pub struct ImplicitCtxt<'tcx> {
     |                ^^^^^^^^^^^^ ----


error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
     |
     |
162  |     rustc_data_structures::sync::assert_sync::<tls::ImplicitCtxt<'_, '_>>();
     |                                                     ^^^^^^^^^^^^     -- help: remove this lifetime argument
     |                                                     expected 1 lifetime argument
     |
     |
note: struct defined here, with 1 lifetime parameter: `'tcx`
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:1706:16
1706 |     pub struct ImplicitCtxt<'tcx> {
     |                ^^^^^^^^^^^^ ----


error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
     |
     |
163  |     let icx: &tls::ImplicitCtxt<'_, '_> = &*(context as *const tls::ImplicitCtxt<'_, '_>);
     |                                                                     ^^^^^^^^^^^^     -- help: remove this lifetime argument
     |                                                                     expected 1 lifetime argument
     |
     |
note: struct defined here, with 1 lifetime parameter: `'tcx`
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:1706:16
1706 |     pub struct ImplicitCtxt<'tcx> {
     |                ^^^^^^^^^^^^ ----

For more information about this error, try `rustc --explain E0107`.

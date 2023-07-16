plain
###                                                                        4.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-09-08/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/oli-obk/tracing.git`
---
    Checking difference v2.0.0
   Compiling coverage_test_macros v0.0.0 (/checkout/compiler/rustc_mir_transform/src/coverage/test_macros)
    Checking odht v0.3.0
    Checking lock_api v0.4.1
    Checking tracing-core v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking thread_local v1.0.1
    Checking sharded-slab v0.1.1
    Checking itertools v0.9.0
    Checking getopts v0.2.21
---
    Checking matchers v0.0.1
    Checking tempfile v3.2.0
    Checking synstructure v0.12.4
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
   Compiling tracing-attributes v0.1.13
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking tracing v0.1.25
    Checking tracing v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
---
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0433]: failed to resolve: could not find `subscriber` in `tracing`
     |
     |
1293 |     tracing::subscriber::set_global_default(subscriber).unwrap();
     |              ^^^^^^^^^^ could not find `subscriber` in `tracing`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_driver` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

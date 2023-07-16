plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-09-08/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/oli-obk/tracing.git`
---
    Checking once_cell v1.7.2
   Compiling coverage_test_macros v0.0.0 (/checkout/compiler/rustc_mir_transform/src/coverage/test_macros)
    Checking odht v0.3.0
    Checking lock_api v0.4.1
    Checking tracing-core v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking thread_local v1.0.1
    Checking sharded-slab v0.1.1
    Checking itertools v0.9.0
    Checking getopts v0.2.21
---
    Checking matchers v0.1.0
    Checking tempfile v3.2.0
    Checking synstructure v0.12.4
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
   Compiling tracing-attributes v0.1.13
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking tracing v0.1.25
    Checking tracing v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking tracing-subscriber v0.3.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
---
    Checking arrayvec v0.7.0
    Checking thread_local v1.0.1
    Checking tracing-core v0.1.17
    Checking sharded-slab v0.1.1
    Checking tracing-core v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking walkdir v2.3.1
   Compiling pest v2.1.3
    Checking itertools v0.9.0
    Checking minifier v0.0.41
---
    Checking ignore v0.4.17
    Checking globwalk v0.8.1
   Compiling pest_generator v2.1.3
   Compiling tracing-attributes v0.1.13
   Compiling tracing-attributes v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking tracing v0.1.25
    Checking tracing v0.1.25
    Checking tracing v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0433]: failed to resolve: could not find `subscriber` in `tracing`
    |
    |
251 |     tracing::subscriber::set_global_default(subscriber).unwrap();
    |              ^^^^^^^^^^ could not find `subscriber` in `tracing`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:56

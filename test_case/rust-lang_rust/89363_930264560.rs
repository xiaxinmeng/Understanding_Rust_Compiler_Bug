plain
                                                                           0.4%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-09-08/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/oli-obk/tracing.git`
---
    Checking difference v2.0.0
   Compiling coverage_test_macros v0.0.0 (/checkout/compiler/rustc_mir_transform/src/coverage/test_macros)
    Checking lock_api v0.4.1
    Checking odht v0.3.0
    Checking tracing-core v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
    Checking sharded-slab v0.1.1
    Checking thread_local v1.0.1
    Checking itertools v0.9.0
    Checking getopts v0.2.21
---
    Checking tempfile v3.2.0
    Checking matchers v0.0.1
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
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0599]: no method named `with_subscriber` found for struct `tracing::Span` in the current scope
   |
   |
68 |         self.0.with_subscriber(|(id, dispatch)| {
   |                ^^^^^^^^^^^^^^^ method not found in `tracing::Span`

error[E0599]: no method named `with_subscriber` found for struct `tracing::Span` in the current scope
   |
   |
76 |         self.0.with_subscriber(|(id, dispatch)| {
   |                ^^^^^^^^^^^^^^^ method not found in `tracing::Span`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_const_eval` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed

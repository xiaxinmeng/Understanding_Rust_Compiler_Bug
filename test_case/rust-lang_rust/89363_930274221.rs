plain
###########################################                               59.7%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-09-08/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
    Updating crates.io index
    Updating git repository `https://github.com/oli-obk/tracing.git`
---
   Compiling fs_extra v1.1.0
   Compiling pathdiff v0.2.0
   Compiling rustc_error_codes v0.0.0 (/checkout/compiler/rustc_error_codes)
   Compiling odht v0.3.0
   Compiling tracing-core v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
   Compiling thread_local v1.0.1
   Compiling sharded-slab v0.1.1
   Compiling lock_api v0.4.1
   Compiling itertools v0.9.0
---
   Compiling rand v0.7.3
   Compiling rustc-rayon v0.3.1
   Compiling tempfile v3.2.0
   Compiling synstructure v0.12.4
   Compiling tracing-attributes v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
   Compiling chalk-derive v0.55.0
   Compiling tracing-attributes v0.1.13
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.25
   Compiling tracing v0.1.25
   Compiling tracing v0.2.0 (https://github.com/oli-obk/tracing.git#5f0154d0)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
   Compiling rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
   Compiling rustc_span v0.0.0 (/checkout/compiler/rustc_span)
---
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
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

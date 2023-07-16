plain
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0391]: cycle detected when optimizing MIR for `rmeta::encoder::<impl at compiler/rustc_metadata/src/rmeta/encoder.rs:987:1: 1955:2>::encode_info_for_mod::{closure#0}`
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1127:73
     |
1127 |             record!(self.tables.children[def_id] <- iter_from_generator(|| {
     |
     |
     = note: ...which immediately requires optimizing MIR for `rmeta::encoder::<impl at compiler/rustc_metadata/src/rmeta/encoder.rs:987:1: 1955:2>::encode_info_for_mod::{closure#0}` again
For more information about this error, try `rustc --explain E0391`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:07:32

plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused import: `FxHashSet`
 --> compiler/rustc_metadata/src/rmeta/encoder.rs:6:44
  |
6 | use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexSet};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0599]: no method named `encode_symbol_table` found for struct `EncodeContext` in the current scope
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:2289:31
     |
     |
49   | pub(super) struct EncodeContext<'a, 'tcx> {
     | ----------------------------------------- method `encode_symbol_table` not found for this struct
...
2289 |     let encoded_symbols = ecx.encode_symbol_table();
     |                               ^^^^^^^^^^^^^^^^^^^ method not found in `EncodeContext<'_, '_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_metadata` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to 2 previous errors

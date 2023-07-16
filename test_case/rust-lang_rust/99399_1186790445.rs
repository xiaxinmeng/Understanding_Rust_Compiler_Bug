plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0432]: unresolved import `rustc_data_structures::stable_map`
 --> compiler/rustc_mir_dataflow/src/un_derefer.rs:1:28
1 | use rustc_data_structures::stable_map::FxHashMap;
  |                            ^^^^^^^^^^ could not find `stable_map` in `rustc_data_structures`

error[E0283]: type annotations needed
error[E0283]: type annotations needed
  --> compiler/rustc_mir_dataflow/src/move_paths/builder.rs:37:66
   |
37 |             un_derefer: UnDerefer { tcx: tcx, derefer_sidetable: Default::default() },
   |
   = note: cannot satisfy `_: std::default::Default`

Some errors have detailed explanations: E0283, E0432.

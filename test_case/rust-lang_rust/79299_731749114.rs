
error[E0658]: use of unstable library feature 'bool_to_option'
   --> compiler/rustc_query_system/src/query/job.rs:416:58
    |
416 |         connected_to_root(query_map, successor, visited).then_some(None)
    |                                                          ^^^^^^^^^
    |
    = note: see issue #64260 <https://github.com/rust-lang/rust/issues/64260> for more information
    = help: add `#![feature(bool_to_option)]` to the crate attributes to enable

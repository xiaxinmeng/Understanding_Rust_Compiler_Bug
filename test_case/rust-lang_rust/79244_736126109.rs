
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: (DefId(0:3866 ~ rustc_middle[1bf6]::ty::codec::TyDecoder), Some(Error#7290))
- dep-node: super_predicates_that_define_assoc_type(5a78152153c9663-dcf2c310262fcf17)', /rustc/349b3b324dade7ca638091db93ba08bbc443c63d/compiler/rustc_query_system/src/query/plumbing.rs:586:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.0.212 (349b3b3 2020-11-29)

query stack during panic:
#0 [fn_sig] computing function signature of `<rustc_index::vec::IndexVec<mir::Promoted, mir::Body<'tcx>> as ty::codec::RefDecodable<'tcx, D>>::decode`
#1 [collect_mod_item_types] collecting item types in module `ty::codec`
end of query stack

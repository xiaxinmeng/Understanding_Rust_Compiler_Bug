
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-58f9efd36de5669ab731ec7ebf565999ff17b159-deep-vector-Opt-IncrUnchanged; results/cgfilt-e6d2de9483a27f846f003fc745713339a9122473-deep-vector-Opt-IncrUnchanged
Command:          /usr/local/rustup/toolchains/58f9efd36de5669ab731ec7ebf565999ff17b159/bin/rustc --crate-name issue_20936_deep_vector src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no -C incremental=/tmp/.tmpqTejpe/incremental-state -C metadata=25fd8fe3e189cc31 -C extra-filename=-25fd8fe3e189cc31 --out-dir /tmp/.tmpqTejpe/target/release/deps -L dependency=/tmp/.tmpqTejpe/target/release/deps -Adeprecated -Aunknown-lints; /usr/local/rustup/toolchains/e6d2de9483a27f846f003fc745713339a9122473/bin/rustc --crate-name issue_20936_deep_vector src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no -C incremental=/tmp/.tmpnAqTP8/incremental-state -C metadata=25fd8fe3e189cc31 -C extra-filename=-25fd8fe3e189cc31 --out-dir /tmp/.tmpnAqTP8/target/release/deps -L dependency=/tmp/.tmpnAqTP8/target/release/deps -Adeprecated -Aunknown-lints
Data file:        results/cgdiff-58f9efd36de5669ab731ec7ebf565999ff17b159-e6d2de9483a27f846f003fc745713339a9122473-deep-vector-Opt-IncrUnchanged
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:     
User annotated:   
Auto-annotation:  on

--------------------------------------------------------------------------------
Ir                   
--------------------------------------------------------------------------------
112,838,555 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                   file:function
--------------------------------------------------------------------------------
31,658,256 (28.06%)  ???:<rustc_data_structures::sip128::SipHasher128>::short_write_process_buffer::<u64>
26,717,744 (23.68%)  ???:<rustc_data_structures::sip128::SipHasher128>::slice_write_process_buffer
14,993,707 (13.29%)  ???:<rustc_span::span_encoding::Span as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
11,579,095 (10.26%)  ???:<rustc_span::caching_source_map_view::CachingSourceMapView>::span_data_to_lines_and_cols
 9,808,597 ( 8.69%)  ???:<&rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
-7,767,094 (-6.88%)  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_const
 6,594,450 ( 5.84%)  ???:<rustc_middle::mir::interpret::value::ConstValue as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
-6,254,706 (-5.54%)  ???:<rustc_middle::mir::Operand as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
 5,437,055 ( 4.82%)  ???:<rustc_middle::ty::sty::TyKind as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 5,166,784 ( 4.58%)  ???:<rustc_middle::mir::Constant as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 4,522,266 ( 4.01%)  ???:<rustc_data_structures::sip128::SipHasher128>::short_write_process_buffer::<u32>
 4,316,982 ( 3.83%)  ???:<&rustc_middle::ty::consts::Const as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 3,399,174 ( 3.01%)  ???:<[rustc_middle::mir::Operand] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
 2,855,457 ( 2.53%)  ???:<rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::mir::Operand>, <alloc::vec::Vec<rustc_middle::mir::Operand> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure
  -815,908 (-0.72%)  ???:<rustc_middle::hir::map::Map>::attrs
   679,859 ( 0.60%)  ???:<rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_expr
   543,882 ( 0.48%)  ???:<rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::<rustc_middle::mir::ConstantKind>
  -408,957 (-0.36%)  ???:rustc_middle::ty::context::tls::TLV::__getit
   135,981 ( 0.12%)  ???:<rustc_middle::ty::TyS as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  -135,969 (-0.12%)  ???:<rustc_lint::early::EarlyContextAndPass<rustc_lint::BuiltinCombinedEarlyLintPass> as rustc_ast::visit::Visitor>::visit_expr

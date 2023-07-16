
--------------------------------------------------------------------------------
Ir                      
--------------------------------------------------------------------------------
19,620,474,127 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                    file:function
--------------------------------------------------------------------------------
715,428,059 ( 3.65%)  ???:rustc_middle::hir::map::Map::attrs
502,838,283 ( 2.56%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
369,884,125 ( 1.89%)  ???:hashbrown::map::RawEntryBuilderMut<K,V,S,A>::from_hash
264,265,957 ( 1.35%)  ???:hashbrown::map::HashMap<K,V,S,A>::insert
253,926,705 ( 1.29%)  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
239,305,386 ( 1.22%)  ???:rustc_middle::ty::context::TyCtxt::_intern_substs
223,580,274 ( 1.14%)  ???:rustc_middle::ty::fold::TypeFoldable::fold_with
182,048,388 ( 0.93%)  ???:rustc_middle::hir::map::Map::find_entry
180,167,164 ( 0.92%)  ???:<smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
169,507,292 ( 0.86%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/mod.rs:core::slice::<impl [T]>::binary_search_by
168,765,087 ( 0.86%)  ???:rustc_middle::ty::context::CtxtInterners::intern_ty
164,041,017 ( 0.84%)  ???:rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with
137,340,433 ( 0.70%)  ???:rustc_hir::intravisit::walk_expr
136,106,538 ( 0.69%)  ???:<rustc_middle::ty::sty::TyKind as core::hash::Hash>::hash
115,206,084 ( 0.59%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/iter/macros.rs:<core::slice::iter::Iter<T> as core::iter::traits::iterator::Iterator>::next
108,704,973 ( 0.55%)  ???:_ZN11rustc_infer5infer9InferCtxt18shallow_resolve_ty17h52a9f2d884c36d9cE.llvm.17265742049258985133
106,081,112 ( 0.54%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/cache_bin.h:free
104,113,685 ( 0.53%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:memcpy@GLIBC_2.2.5
101,631,244 ( 0.52%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/strnom.rs:proc_macro2::strnom::whitespace
 95,861,760 ( 0.49%)  ???:rustc_data_structures::sip128::SipHasher128::short_write_process_buffer
 95,161,483 ( 0.49%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/cache_bin.h:malloc
 91,567,848 ( 0.47%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/rtree.h:free
 91,415,495 ( 0.47%)  ???:rustc_mir::dataflow::drop_flag_effects::drop_flag_effects_for_location
 90,261,181 ( 0.46%)  ???:_ZN21rustc_data_structures17obligation_forest25ObligationForest$LT$O$GT$22register_obligation_at17h76d03e3dc4332819E.llvm.6990287777164569665
 90,062,366 ( 0.46%)  ???:rustc_data_structures::graph::scc::SccsConstruction<G,S>::start_walk_from
 88,225,511 ( 0.45%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/alloc/src/slice.rs:<T as alloc::slice::hack::ConvertVec>::to_vec
 88,175,036 ( 0.45%)  ???:_ZN85_$LT$rustc_infer..infer..region_constraints..Constraint$u20$as$u20$core..cmp..Ord$GT$3cmp17hbabfa2ba4084c03aE.llvm.16422726908259070308
 86,556,584 ( 0.44%)  ???:<alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
 81,342,205 ( 0.41%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/cmp.rs:<core::cmp::Ordering as core::cmp::PartialEq>::eq
 80,499,115 ( 0.41%)  ???:_ZN21rustc_data_structures17obligation_forest25ObligationForest$LT$O$GT$8compress17heb8863a20d5068e2E.llvm.6990287777164569665
 78,640,839 ( 0.40%)  ???:rustc_middle::ty::relate::super_relate_tys
 77,305,558 ( 0.39%)  ???:<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
 76,121,214 ( 0.39%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/jemalloc.c:free
 74,154,161 ( 0.38%)  ???:rustc_middle::ty::context::TyCtxt::mk_region
 74,103,648 ( 0.38%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.1.0/src/tables.rs:unicode_xid::tables::bsearch_range_table::{{closure}}
 72,674,179 ( 0.37%)  ???:scoped_tls::ScopedKey<T>::with
 71,839,782 ( 0.37%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/mod.rs:core::slice::<impl [T]>::len
 71,655,996 ( 0.37%)  ???:_ZN12rustc_middle3mir9traversal9Postorder18traverse_successor17hed1b531099968f53E.llvm.11822269122503392289
 69,332,156 ( 0.35%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/iter/adapters/enumerate.rs:<core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::next
 68,620,360 ( 0.35%)  ???:rustc_index::bit_set::HybridBitSet<T>::insert
 68,595,296 ( 0.35%)  ???:rustc_middle::ty::context::CtxtInterners::intern_predicate
 67,581,507 ( 0.34%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/index.rs:<usize as core::slice::index::SliceIndex<[T]>>::get_unchecked
 67,261,007 ( 0.34%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/jemalloc.c:malloc
 65,808,124 ( 0.34%)  ???:<rustc_middle::ty::sty::RegionKind as core::hash::Hash>::hash
 65,050,242 ( 0.33%)  ???:<rustc_mir::dataflow::framework::direction::Forward as rustc_mir::dataflow::framework::direction::Direction>::join_state_into_successors_of
 62,658,418 ( 0.32%)  ???:rustc_infer::infer::region_constraints::RegionConstraintCollector::make_subregion
 58,849,103 ( 0.30%)  ???:rustc_data_structures::sso::map::SsoHashMap<K,V>::insert
 58,188,448 ( 0.30%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/arena.c:arena_dalloc_bin_locked_impl
 57,275,191 ( 0.29%)  ???:_ZN70_$LT$rustc_middle..ty..sty..TyKind$u20$as$u20$core..cmp..PartialEq$GT$2eq17hf40419efeba02a7bE.llvm.10868335851582077806
 56,857,372 ( 0.29%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/traits.rs:core::str::traits::<impl core::slice::index::SliceIndex<str> for core::ops::range::RangeFrom<usize>>::index
 54,013,976 ( 0.28%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/mod.rs:core::slice::<impl [T]>::starts_with
 53,065,872 ( 0.27%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/mod.rs:core::str::<impl str>::len
 52,655,939 ( 0.27%)  ???:rustc_middle::ty::query::<impl rustc_middle::ty::context::TyCtxt>::def_kind
 52,559,941 ( 0.27%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f//library/std/src/sys/unix/alloc.rs:__rdl_alloc
 52,289,512 ( 0.27%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/option.rs:core::option::Option<T>::ok_or
 52,221,659 ( 0.27%)  ???:<rustc_middle::ty::PredicateKind as core::hash::Hash>::hash
 51,904,390 ( 0.26%)  ???:rustc_middle::ty::util::fold_list
 51,688,520 ( 0.26%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/traits.rs:core::str::traits::<impl core::slice::index::SliceIndex<str> for core::ops::range::RangeFrom<usize>>::get_unchecked
 49,928,265 ( 0.25%)  ???:_ZN9rustc_mir12borrow_check15MirBorrowckCtxt12access_place17h8abfbfd9542d91b0E.llvm.12074767688786621785
 49,871,005 ( 0.25%)  ???:rustc_middle::middle::region::ScopeTree::temporary_scope
 49,856,308 ( 0.25%)  ???:rustc_middle::ty::context::TyCtxt::maybe_lint_level_root_bounded
 49,394,127 ( 0.25%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/cmp.rs:<[A] as core::slice::cmp::SlicePartialEq<B>>::equal
 49,259,718 ( 0.25%)  ???:rustc_mir::transform::simplify::CfgSimplifier::simplify
 49,139,116 ( 0.25%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/mut_ptr.rs:core::ptr::mut_ptr::<impl *mut T>::is_null
 48,964,085 ( 0.25%)  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_middle::ty::fold::TypeFolder>::fold_ty
 48,704,430 ( 0.25%)  ???:rustc_lexer::<impl rustc_lexer::cursor::Cursor>::advance_token
 47,945,585 ( 0.24%)  ???:smallvec::SmallVec<A>::try_reserve
 47,928,501 ( 0.24%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/arena.c:_rjem_je_arena_ralloc
 47,797,422 ( 0.24%)  ???:_ZN74_$LT$rustc_middle..ty..sty..RegionKind$u20$as$u20$core..cmp..PartialEq$GT$2eq17h2fbb4016682082c0E.llvm.15156200988390750590
 46,953,476 ( 0.24%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/lib.rs:<proc_macro2::TokenTree as core::clone::Clone>::clone
 46,860,226 ( 0.24%)  ???:<&mut U as ena::undo_log::UndoLogs<T>>::push
 45,531,779 ( 0.23%)  ???:<alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
 45,359,184 ( 0.23%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/mut_ptr.rs:core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
 44,784,708 ( 0.23%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/sz.h:malloc
 43,989,981 ( 0.22%)  ???:rustc_mir::dataflow::framework::engine::Engine<A>::iterate_to_fixpoint
 43,904,233 ( 0.22%)  ???:<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
 43,779,452 ( 0.22%)  ???:_ZN11rustc_index7bit_set21HybridBitSet$LT$T$GT$5union17hafadeda2f8e94de0E.llvm.5901171670611942340
 43,319,428 ( 0.22%)  ???:alloc::vec::Vec<T,A>::extend_with
 43,275,165 ( 0.22%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/tcache.c:_rjem_je_tcache_bin_flush_small
 42,651,719 ( 0.22%)  ???:<A as rustc_mir::dataflow::framework::Analysis>::apply_statement_effect
 42,643,029 ( 0.22%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/traits.rs:core::str::traits::<impl core::slice::index::SliceIndex<str> for core::ops::range::RangeFrom<usize>>::get
 42,618,955 ( 0.22%)  ???:_ZN12rustc_middle2ty7context6TyCtxt21reuse_or_mk_predicate17hddb5fa396d361dcdE.llvm.3860820800893818612
 42,302,281 ( 0.22%)  ???:rustc_hir::intravisit::walk_qpath
 42,176,996 ( 0.21%)  ???:rustc_parse::lexer::StringReader::next_token
 42,158,773 ( 0.21%)  ???:rustc_query_system::query::plumbing::JobOwner<D,C>::complete
 41,854,627 ( 0.21%)  ???:rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations
 41,493,773 ( 0.21%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/mod.rs:core::slice::<impl [T]>::get_unchecked
 41,381,184 ( 0.21%)  ???:rustc_infer::infer::InferCtxt::rollback_to
 40,577,573 ( 0.21%)  ???:alloc::vec::source_iter_marker::<impl alloc::vec::spec_from_iter::SpecFromIter<T,I> for alloc::vec::Vec<T>>::from_iter
 40,373,381 ( 0.21%)  ???:rustc_parse::parser::Parser::bump
 40,340,616 ( 0.21%)  ???:rustc_infer::infer::InferCtxt::start_snapshot
 40,211,595 ( 0.20%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/mod.rs:core::ptr::slice_from_raw_parts
 39,838,291 ( 0.20%)  ???:rustc_resolve::Resolver::resolve_ident_in_lexical_scope
 39,655,832 ( 0.20%)  ???:rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations
 39,576,290 ( 0.20%)  ???:<rustc_privacy::DefIdVisitorSkeleton<V> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
 39,517,159 ( 0.20%)  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::apply_rewrites
 39,146,172 ( 0.20%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/const_ptr.rs:core::ptr::const_ptr::<impl *const T>::is_null
 38,876,603 ( 0.20%)  ???:rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_visit_with
 38,783,414 ( 0.20%)  ???:rustc_infer::infer::region_constraints::RegionConstraintCollector::new_region_var
 38,633,124 ( 0.20%)  ???:rustc_query_system::query::plumbing::force_query_with_job
 38,241,607 ( 0.19%)  ???:rustc_mir::borrow_check::type_check::liveness::trace::trace
 38,129,636 ( 0.19%)  ???:rustc_ast::visit::walk_expr
 37,806,602 ( 0.19%)  ???:rustc_infer::infer::InferCtxt::commit_if_ok
 37,736,685 ( 0.19%)  ???:_ZN13rustc_privacy18TypePrivacyVisitor18item_is_accessible17h8afe9d6f959144dcE.llvm.211790508500069708
 36,530,915 ( 0.19%)  ???:rustc_infer::infer::canonical::canonicalizer::Canonicalizer::canonicalize
 36,492,172 ( 0.19%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/non_null.rs:core::ptr::non_null::NonNull<T>::as_ptr
 36,276,103 ( 0.18%)  ???:rustc_trait_selection::traits::select::SelectionContext::match_impl
 36,209,036 ( 0.18%)  ???:<rustc_middle::ty::walk::TypeWalker as core::iter::traits::iterator::Iterator>::next
 36,134,928 ( 0.18%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/const_ptr.rs:core::ptr::const_ptr::<impl *const T>::guaranteed_eq
 36,090,732 ( 0.18%)  ???:rustc_mir::borrow_check::type_check::TypeChecker::typeck_mir
 35,664,494 ( 0.18%)  ???:_ZN11rustc_infer5infer22lexical_region_resolve15LexicalResolver21infer_variable_values17hd88b87141835caabE.llvm.14357165609930724401
 35,628,414 ( 0.18%)  ???:rustc_middle::ty::adt::AdtDef::destructor
 35,562,150 ( 0.18%)  ???:rustc_middle::ty::context::TyCtxt::lang_items
 35,333,419 ( 0.18%)  ???:_ZN21rustc_data_structures5graph3scc29SccsConstruction$LT$G$C$S$GT$12inspect_node17h8335fa008af88126E.llvm.5901171670611942340
 35,085,126 ( 0.18%)  ???:<alloc::vec::Vec<T,A> as core::clone::Clone>::clone
 34,708,196 ( 0.18%)  ???:<rustc_middle::mir::traversal::Preorder as core::iter::traits::iterator::Iterator>::next
 34,696,814 ( 0.18%)  ???:rustc_span::<impl rustc_data_structures::stable_hasher::HashStable<CTX> for rustc_span::span_encoding::Span>::hash_stable
 34,051,633 ( 0.17%)  ???:rustc_hir::intravisit::walk_path
 33,968,405 ( 0.17%)  ???:alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut,K,V,alloc::collections::btree::node::marker::Leaf>,alloc::collections::btree::node::marker::Edge>::insert_recursing
 32,844,532 ( 0.17%)  ???:<I as rustc_middle::ty::context::InternAs<[T],R>>::intern_with
 31,765,104 ( 0.16%)  ???:rustc_query_system::cache::Cache<Key,Value>::get
 31,281,916 ( 0.16%)  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
 31,109,545 ( 0.16%)  ???:<&mut alloc::vec::Vec<<D as ena::snapshot_vec::SnapshotVecDelegate>::Value> as ena::snapshot_vec::VecLike<D>>::push
 31,046,347 ( 0.16%)  ???:_ZN21rustc_trait_selection6traits6select18candidate_assembly73_$LT$impl$u20$rustc_trait_selection..traits..select..SelectionContext$GT$25candidate_from_obligation17h297110f12e61eb56E.llvm.11581445551101323097
 30,641,879 ( 0.16%)  ???:rustc_hir::intravisit::walk_pat
 30,495,972 ( 0.16%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/ticker.h:free
 30,232,152 ( 0.15%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/iter/adapters/take.rs:<core::iter::adapters::take::Take<I> as core::iter::traits::iterator::Iterator>::next
 30,140,854 ( 0.15%)  ???:<rustc_infer::infer::combine::Generalizer as rustc_middle::ty::relate::TypeRelation>::tys
 30,046,792 ( 0.15%)  ???:<alloc::vec::Vec<T,A> as core::iter::traits::collect::Extend<&T>>::extend
 29,986,968 ( 0.15%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/ticker.h:malloc
 29,741,599 ( 0.15%)  ???:rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::span_encoding::Span>::decode
 29,488,503 ( 0.15%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/metadata.rs:core::ptr::metadata::from_raw_parts
 29,441,336 ( 0.15%)  ???:rustc_middle::ty::walk::push_inner
 29,380,021 ( 0.15%)  ???:rustc_infer::infer::combine::CombineFields::instantiate
 29,261,781 ( 0.15%)  ???:rustc_mir::borrow_check::region_infer::RegionInferenceContext::new
 28,746,943 ( 0.15%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/mem/maybe_uninit.rs:<T as alloc::slice::hack::ConvertVec>::to_vec
 28,651,473 ( 0.15%)  ???:hashbrown::raw::RawTableInner<A>::prepare_resize
 28,648,856 ( 0.15%)  ???:rustc_index::bit_set::SparseBitMatrix<R,C>::union_rows
 28,544,750 ( 0.15%)  ???:rustc_span::caching_source_map_view::CachingSourceMapView::span_data_to_lines_and_cols
 28,298,274 ( 0.14%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/mod.rs:core::str::<impl str>::is_char_boundary
 28,238,365 ( 0.14%)  ???:rustc_data_structures::graph::dominators::dominators
 28,022,272 ( 0.14%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/index.rs:<core::ops::range::Range<usize> as core::slice::index::SliceIndex<[T]>>::index
 27,867,993 ( 0.14%)  ???:<core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold
 27,653,379 ( 0.14%)  ???:rustc_span::source_map::SourceMap::lookup_byte_offset
 27,611,715 ( 0.14%)  ???:rustc_middle::ty::context::TyCtxt::_intern_type_list
 27,531,810 ( 0.14%)  ???:stacker::remaining_stack
 27,410,458 ( 0.14%)  ???:_ZN11rustc_infer5infer9canonical13canonicalizer13Canonicalizer13canonical_var17h65cbe081d1e5ac27E.llvm.14357165609930724401
 27,215,752 ( 0.14%)  ???:rustc_middle::ty::context::TypeckResults::node_type
 27,198,572 ( 0.14%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/arena.c:_rjem_je_arena_tcache_fill_small
 27,165,806 ( 0.14%)  ???:_ZN13rustc_resolve8Resolver16traits_in_module17he5e2d2892aa99b88E.llvm.12035942568840590830
 27,072,192 ( 0.14%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/strnom.rs:proc_macro2::strnom::Cursor::advance
 26,956,161 ( 0.14%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/strnom.rs:proc_macro2::strnom::skip_whitespace
 26,708,174 ( 0.14%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/iter.rs:core::slice::iter::Iter<T>::new
 26,410,116 ( 0.13%)  ???:rustc_data_structures::stack::ensure_sufficient_stack
 26,192,963 ( 0.13%)  ???:rustc_typeck::check::writeback::WritebackCx::visit_node_id
 26,079,783 ( 0.13%)  ???:rustc_infer::infer::type_variable::TypeVariableTable::new_var
 25,995,373 ( 0.13%)  ???:<alloc::string::String as core::iter::traits::collect::Extend<char>>::extend
 25,788,011 ( 0.13%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/jemalloc.c:realloc
 25,786,934 ( 0.13%)  ???:alloc::collections::btree::map::BTreeMap<K,V>::remove
 25,779,080 ( 0.13%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/mod.rs:<&str as core::str::pattern::Pattern>::is_prefix_of
 25,598,465 ( 0.13%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/src/arena.c:_rjem_je_arena_ralloc_no_move
 25,412,845 ( 0.13%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/index.rs:<core::ops::range::Range<usize> as core::slice::index::SliceIndex<[T]>>::get_unchecked
 25,256,059 ( 0.13%)  ???:rustc_mir::borrow_check::path_utils::each_borrow_involving_path
 25,236,470 ( 0.13%)  ???:rustc_mir::borrow_check::region_infer::RegionInferenceContext::solve
 25,205,785 ( 0.13%)  ???:rustc_infer::infer::InferCtxt::freshener
 25,086,523 ( 0.13%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/slice/sort.rs:core::slice::sort::recurse
 25,056,299 ( 0.13%)  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
 24,958,656 ( 0.13%)  ???:rustc_trait_selection::traits::query::type_op::QueryTypeOp::fully_perform_into
 24,918,113 ( 0.13%)  ???:alloc::vec::Vec<T,A>::truncate
 24,757,334 ( 0.13%)  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
 24,731,750 ( 0.13%)  ???:<rustc_mir::dataflow::framework::direction::Forward as rustc_mir::dataflow::framework::direction::Direction>::visit_results_in_block
 24,644,684 ( 0.13%)  ???:_ZN9rustc_mir8dataflow17drop_flag_effects20on_all_children_bits20on_all_children_bits17h4ea07005ad50b215E.llvm.17292416061430652635
 24,460,910 ( 0.12%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/mod.rs:core::ptr::drop_in_place<proc_macro2::TokenTree>
 24,400,327 ( 0.12%)  ???:rustc_mir_build::thir::pattern::usefulness::is_useful
 24,083,391 ( 0.12%)  ???:rustc_infer::infer::InferCtxt::commit_from
 23,804,589 ( 0.12%)  ???:<rustc_infer::infer::combine::Generalizer as rustc_middle::ty::relate::TypeRelation>::relate_item_substs
 23,800,269 ( 0.12%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/strnom.rs:proc_macro2::strnom::punct
 23,637,381 ( 0.12%)  ???:<alloc::rc::Rc<T> as core::ops::drop::Drop>::drop
 23,616,791 ( 0.12%)  ???:<rustc_infer::infer::sub::Sub as rustc_middle::ty::relate::TypeRelation>::tys
 23,232,933 ( 0.12%)  ???:_ZN70_$LT$rustc_middle..ty..sty..TyKind$u20$as$u20$core..cmp..PartialEq$GT$2eq17hf40419efeba02a7bE.llvm.7948815897050953357
 23,200,314 ( 0.12%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/mod.rs:core::str::<impl str>::starts_with
 22,948,175 ( 0.12%)  ???:rustc_lexer::cursor::Cursor::nth_char
 22,931,826 ( 0.12%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/mod.rs:core::ptr::drop_in_place<[proc_macro2::TokenTree]>
 22,766,991 ( 0.12%)  ???:indexmap::map::IndexMap<K,V,S>::entry
 22,724,801 ( 0.12%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/sz.h:_rjem_je_arena_ralloc_no_move
 22,498,430 ( 0.11%)  ???:_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17hfc744650a29e6fd0E.llvm.8124594429463474725
 22,362,750 ( 0.11%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/lib.rs:<proc_macro2::Punct as core::clone::Clone>::clone
 22,273,357 ( 0.11%)  ???:_ZN15rustc_mir_build5build4expr4into49_$LT$impl$u20$rustc_mir_build..build..Builder$GT$14expr_into_dest17h5acfcaaf7ff8652fE.llvm.9230732149122922343
 21,777,879 ( 0.11%)  ???:rustc_parse::parser::TokenCursor::next
 21,653,612 ( 0.11%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/ptr/const_ptr.rs:core::ptr::const_ptr::<impl *const [T]>::as_ptr
 21,581,430 ( 0.11%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-aa6f08d78381239a/out/build/include/jemalloc/internal/rtree.h:_rjem_je_tcache_bin_flush_small
 21,402,020 ( 0.11%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/option.rs:core::option::Option<T>::map
 21,257,272 ( 0.11%)  ???:rustc_trait_selection::traits::project::AssocTypeNormalizer::fold
 21,131,560 ( 0.11%)  ???:rustc_infer::infer::canonical::query_response::<impl rustc_infer::infer::InferCtxt>::instantiate_query_response_and_region_obligations
 21,120,058 ( 0.11%)  ???:_ZN13rustc_resolve7imports41_$LT$impl$u20$rustc_resolve..Resolver$GT$38resolve_ident_in_module_unadjusted_ext17hba294c2de2b7e70cE.llvm.12035942568840590830
 20,961,865 ( 0.11%)  ???:_ZN21rustc_data_structures17obligation_forest25ObligationForest$LT$O$GT$21find_cycles_from_node17h4a1eb71c3352fd82E.llvm.6990287777164569665
 20,951,318 ( 0.11%)  ???:_ZN11rustc_infer5infer2at2At7sub_exp17h898c2eca5d885d23E.llvm.2362979508110025503
 20,862,715 ( 0.11%)  ???:_ZN13rustc_resolve6macros41_$LT$impl$u20$rustc_resolve..Resolver$GT$36early_resolve_ident_in_lexical_scope17h01076e04c22510a3E.llvm.12035942568840590830
 20,799,106 ( 0.11%)  ???:core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
 20,692,920 ( 0.11%)  ???:_ZN21rustc_trait_selection6traits2wf12WfPredicates9normalize17hcefb70b071d8fa19E.llvm.7716279718124322772
 20,651,029 ( 0.11%)  ???:<core::result::Result<T,E> as rustc_middle::ty::context::InternIteratorElement<T,R>>::intern_with
 20,623,264 ( 0.11%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/pattern.rs:<&str as core::str::pattern::Pattern>::is_prefix_of
 20,610,944 ( 0.11%)  ~/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.3/src/strnom.rs:proc_macro2::strnom::Cursor::starts_with
 20,580,950 ( 0.10%)  ???:_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h1f1e519a4b84c482E.llvm.8124594429463474725
 20,485,333 ( 0.10%)  ???:rustc_ast_lowering::path::<impl rustc_ast_lowering::LoweringContext>::lower_path_segment
 20,484,520 ( 0.10%)  ./elf/dl-lookup.c:_dl_lookup_symbol_x
 20,304,472 ( 0.10%)  ???:rustc_infer::infer::undo_log::<impl ena::undo_log::Rollback<rustc_infer::infer::undo_log::UndoLog> for rustc_infer::infer::InferCtxtInner>::reverse
 20,292,776 ( 0.10%)  ???:rustc_parse::lexer::tokentrees::TokenTreesReader::parse_token_tree
 20,083,117 ( 0.10%)  ???:_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h51b59a205201fe12E.llvm.5089112490155980438
 20,039,688 ( 0.10%)  ???:rustc_middle::ty::fold::<impl rustc_middle::ty::context::TyCtxt>::replace_escaping_bound_vars
 19,994,245 ( 0.10%)  ???:alloc::collections::btree::map::entry::Entry<K,V>::or_insert_with
 19,834,333 ( 0.10%)  ???:<rustc_mir::dataflow::framework::direction::Forward as rustc_mir::dataflow::framework::direction::Direction>::apply_effects_in_block
 19,831,289 ( 0.10%)  /rustc/8b83f733fef262fda5a9bda1873ca8070bc8531f/library/core/src/str/traits.rs:core::str::traits::<impl core::ops::index::Index<I> for str>::index
 19,808,801 ( 0.10%)  ???:<rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr
 19,769,639 ( 0.10%)  ???:rustc_hir::intravisit::walk_ty
 19,730,883 ( 0.10%)  ???:rustc_middle::ty::flags::FlagComputation::for_kind

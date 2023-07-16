
 cg_diff cgout-{before,after}-regression-31157-Doc-Full | sed 's@e4828d5b7f745a9e867a9b0cc7f080f287bcf55d@@' | sed 's@eec63217b0c2bd2bd82596fe0f8f6ccc5b9cc1a0@@' | rustfilt | cg_annotate /dev/stdin
--------------------------------------------------------------------------------
Files compared:   cgout-before-regression-31157-Doc-Full; cgout-after-regression-31157-Doc-Full
Command:          /home/joshua/.local/lib/rustup/toolchains//bin/rustdoc --crate-type lib --crate-name parser src/lib.rs -o /home/joshua/.local/lib/cargo/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/.local/lib/cargo/target/debug/deps --extern peruse=/home/joshua/.local/lib/cargo/target/debug/deps/libperuse-d114ff6eeca80a88.rmeta --crate-version 0.1.0 -Adeprecated -Aunknown-lints; /home/joshua/.local/lib/rustup/toolchains//bin/rustdoc --crate-type lib --crate-name parser src/lib.rs -o /home/joshua/.local/lib/cargo/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/.local/lib/cargo/target/debug/deps --extern peruse=/home/joshua/.local/lib/cargo/target/debug/deps/libperuse-d114ff6eeca80a88.rmeta --crate-version 0.1.0 -Adeprecated -Aunknown-lints
Data file:        /dev/stdin
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:     
User annotated:   
Auto-annotation:  off

--------------------------------------------------------------------------------
Ir          
--------------------------------------------------------------------------------
166,969,656  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir          file:function
--------------------------------------------------------------------------------
31,502,496  ???:rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::span_encoding::Span>::decode
 7,653,789  ???:rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_metadata::creader::CStore>::item_children_untracked
 6,913,556  ???:rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::hygiene::SyntaxContext>::decode
 6,608,013  ???:rustc_resolve::imports::<impl rustc_resolve::Resolver>::try_define
 5,902,636  ???:indexmap::map::core::Entry<K,V>::or_insert_with
 5,805,920  ???:rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::imported_source_files
 5,553,403  ???:rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor::build_reduced_graph_for_external_crate_res
 5,047,864  ???:hashbrown::map::HashMap<K,V,S,A>::insert
 4,886,330  ???:hashbrown::raw::RawTable<T,A>::reserve_rehash
 4,742,352  ???:indexmap::map::IndexMap<K,V,S>::entry
 4,470,310  ???:<rustc_span::def_id::DefId as rustc_serialize::serialize::Decodable<D>>::decode
 4,223,606  ???:scoped_tls::ScopedKey<T>::with
 4,015,378  ???:rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::define
 3,988,003  /rustc///library/core/src/str/validations.rs:core::str::converts::from_utf8
 3,711,738  ???:rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::def_key
 3,604,660  ???:rustc_resolve::Resolver::resolutions
 3,230,864  ???:rustc_span::symbol::Ident::normalize_to_macros_2_0
 3,001,575  ???:hashbrown::raw::inner::RawTable<T,A>::reserve_rehash
 2,992,579  ???:rustc_middle::hir::exports::_DERIVE_rustc_serialize_Decodable_D_FOR_Export::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::hir::exports::Export>::decode
 2,842,145  ???:rustc_metadata::rmeta::table::<impl rustc_metadata::rmeta::Lazy<rustc_metadata::rmeta::table::Table<I,T>>>::get
 2,467,516  ???:core::lazy::OnceCell<T>::get_or_init
 2,388,161  ???:rustc_metadata::rmeta::_DERIVE_rustc_serialize_Decodable_DecodeContext_a_tcx_FOR_EntryKind::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_metadata::rmeta::EntryKind>::decode
 2,352,269  ???:rustc_hir::def::_DERIVE_rustc_serialize_Decodable_D_FOR_DefKind::<impl rustc_serialize::serialize::Decodable<__D> for rustc_hir::def::DefKind>::decode
 2,233,897  ???:rustc_hir::def::_DERIVE_rustc_serialize_Decodable_D_FOR_Res::<impl rustc_serialize::serialize::Decodable<__D> for rustc_hir::def::Res<Id>>::decode
 2,144,697  ???:rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::try_item_ident
 2,050,836  ???:rustc_resolve::Resolver::traits_in_module
 1,993,505  ???:rustc_metadata::rmeta::decoder::__ty_decoder_impl::<impl rustc_serialize::serialize::Decoder for rustc_metadata::rmeta::decoder::DecodeContext>::read_str
 1,976,679  ???:rustc_serialize::serialize::Decoder::read_option
 1,847,980  ???:rustc_middle::ty::_DERIVE_rustc_serialize_Decodable_D_FOR_Visibility::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::ty::Visibility>::decode
 1,845,837  ???:<core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
 1,660,508  ???:<(T10,T11) as rustc_serialize::serialize::Decodable<D>>::decode
 1,557,738  ???:rustc_span::hygiene::HygieneData::with
 1,401,608  ???:rustc_metadata::rmeta::decoder::DecodeContext::read_lazy_with_meta
 1,311,606  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
 1,096,224  ???:rustc_resolve::macros::<impl rustc_resolve::Resolver>::check_reserved_macro_name
 1,055,352  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/multiarch/memcmp-avx2-movbe.S:__memcmp_avx2_movbe
   836,472  ???:<core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
   796,983  ???:rustdoc::passes::collect_intra_doc_links::early::IntraLinkCrateLoader::add_traits_in_scope
   714,315  ???:rustc_span::SESSION_GLOBALS::FOO::__getit
   710,028  ???:hashbrown::raw::RawTable<T,A>::insert
   678,640  /rustc///library/core/src/str/converts.rs:core::str::converts::from_utf8
   637,346  ???:rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::get_span
   611,723  ???:<alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
   600,778  ???:rustc_middle::ty::fast_reject::_DERIVE_rustc_serialize_Decodable_D_FOR_SimplifiedTypeGen::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::ty::fast_reject::SimplifiedTypeGen<D>>::decode
   541,236  ???:rustc_middle::mir::query::_DERIVE_rustc_serialize_Decodable_D_FOR_ConstQualifs::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::mir::query::ConstQualifs>::decode
   529,200  ???:rustc_metadata::rmeta::decoder::<impl rustc_metadata::rmeta::Lazy<T>>::decode
   439,146  /build/glibc-eX1tMB/glibc-2.31/string/../sysdeps/x86_64/multiarch/memset-vec-unaligned-erms.S:__memset_avx2_erms
   419,874  ???:rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_metadata::creader::CStore>::ctor_def_id_and_kind_untracked
   408,310  ???:rustc_hir::definitions::_DERIVE_rustc_serialize_Decodable_D_FOR_DefKey::<impl rustc_serialize::serialize::Decodable<__D> for rustc_hir::definitions::DefKey>::decode
   401,632  ???:<&rustc_resolve::Resolver as rustc_middle::ty::DefIdTree>::parent
   373,248  ???:<rustc_arena::TypedArena<T> as core::ops::drop::Drop>::drop
   369,502  /rustc///library/core/src/num/uint_macros.rs:core::str::converts::from_utf8
   339,320  ???:rustc_span::symbol::Symbol::intern
   331,632  ???:rustc_middle::ty::_DERIVE_rustc_serialize_Decodable_D_FOR_VariantDiscr::<impl rustc_serialize::serialize::Decodable<__D> for rustc_middle::ty::VariantDiscr>::decode
   323,634  ???:core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
   295,152  /rustc//obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/tikv-jemalloc-sys-fde45a9a697cc078/out/build/src/arena.c:_rjem_je_arena_ralloc
   274,104  ???:rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::def_key
   267,978  ???:rustc_hir::definitions::DefPathData::get_opt_name
  -262,298  ???:alloc::str::<impl str>::replace
   253,800  ???:rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_metadata::creader::CStore>::struct_field_names_untracked
   241,850  ???:core::str::pattern::TwoWaySearcher::next
   230,640  ???:rustdoc::passes::collect_intra_doc_links::early::load_intra_link_crates
   189,924  ???:rustc_metadata::rmeta::decoder::<impl rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext> for rustc_span::def_id::CrateNum>::decode
   172,159  ???:core::ptr::drop_in_place<[rustdoc::clean::types::Item]>

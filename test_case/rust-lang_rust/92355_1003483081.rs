
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (f8abed9ed 2021-12-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=256 -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `hacks::<impl at bwmapserver/src/hacks.rs:563:1: 563:28>::register::extract_chk_from_mpqs_where_chk_is_null`
#1 [analysis] running analysis passes on this crate
end of query stack

... snip ....
  19:     0x7f0822607c1b - core::panicking::panic_str::hde62b5b8a6a733ac
                               at /rustc/f8abed9ed48bace6be0087bcd44ed534e239b8d8/library/core/src/panicking.rs:55:5
  20:     0x7f0822576ca6 - core::option::expect_failed::h92ed43bc5e397c2e
                               at /rustc/f8abed9ed48bace6be0087bcd44ed534e239b8d8/library/core/src/option.rs:1821:5
  21:     0x7f08246cd023 - <rustc_span[607f9cf87f924056]::span_encoding::Span as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7f08246c9e60 - <rustc_middle[c579614bfb1210b6]::ty::VariantDef as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  23:     0x7f082468d4d7 - <rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder as rustc_serialize[51237e0b41fcf1f2]::serialize::Decoder>::read_seq::<alloc[8ecd766e14532564]::vec::Vec<rustc_middle[c579614bfb1210b6]::ty::VariantDef>, <alloc[8ecd766e14532564]::vec::Vec<rustc_middle[c579614bfb1210b6]::ty::VariantDef> as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  24:     0x7f0825114907 - <rustc_middle[c579614bfb1210b6]::ty::adt::AdtDef as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  25:     0x7f08246725ee - <rustc_middle[c579614bfb1210b6]::ty::sty::TyKind as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  26:     0x7f08246c22d0 - <&rustc_middle[c579614bfb1210b6]::ty::TyS as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  27:     0x7f0824672734 - <rustc_middle[c579614bfb1210b6]::ty::sty::TyKind as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  28:     0x7f08246c22d0 - <&rustc_middle[c579614bfb1210b6]::ty::TyS as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  29:     0x7f08246c219a - <&rustc_middle[c579614bfb1210b6]::ty::TyS as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode
  30:     0x7f0824692186 - <rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder as rustc_serialize[51237e0b41fcf1f2]::serialize::Decoder>::read_map::<std[c9dd2fc8e45fffc5]::collections::hash::map::HashMap<rustc_hir[add2b00c28112c82]::hir_id::ItemLocalId, &rustc_middle[c579614bfb1210b6]::ty::TyS, core[570de0aec9df84a9]::hash::BuildHasherDefault<rustc_hash[a3c3ba9fc61adc85]::FxHasher>>, <std[c9dd2fc8e45fffc5]::collections::hash::map::HashMap<rustc_hir[add2b00c28112c82]::hir_id::ItemLocalId, &rustc_middle[c579614bfb1210b6]::ty::TyS, core[570de0aec9df84a9]::hash::BuildHasherDefault<rustc_hash[a3c3ba9fc61adc85]::FxHasher>> as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
  31:     0x7f0825121801 - <rustc_middle[c579614bfb1210b6]::ty::context::TypeckResults as rustc_serialize[51237e0b41fcf1f2]::serialize::Decodable<rustc_query_impl[731ad525f14f9d89]::on_disk_cache::CacheDecoder>>::decode::{closure#0}
  32:     0x7f082513efb8 - <rustc_query_impl[731ad525f14f9d89]::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle[c579614bfb1210b6]::ty::context::TypeckResults>
  33:     0x7f082507aef6 - rustc_query_system[47f2870a8b951c1b]::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl[731ad525f14f9d89]::plumbing::QueryCtxt, rustc_span[607f9cf87f924056]::def_id::LocalDefId, &rustc_middle[c579614bfb1210b6]::ty::context::TypeckResults>
.... snip ....


--------------------------------------------------------------------------------
Ir                 
--------------------------------------------------------------------------------
9,063,571 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                   file:function
--------------------------------------------------------------------------------
-1,355,130 (-15.0%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:serialize::serialize::Decoder::read_struct_field
   899,266 ( 9.92%)  /home/njn/moz/rustN/src/librustc_metadata/decoder.rs:rustc_metadata::decoder::DecodeContext::read_lazy_distance
   772,315 ( 8.52%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:<rustc_metadata::schema::Entry as serialize::serialize::Decodable>::decode
   647,102 ( 7.14%)  /home/njn/moz/rustN/src/libstd/collections/hash/table.rs:rustc::dep_graph::prev::PreviousDepGraph::new
   575,554 ( 6.35%)  /home/njn/moz/rustN/src/libcore/ptr.rs:rustc::dep_graph::prev::PreviousDepGraph::new
   493,118 ( 5.44%)  /home/njn/moz/rustN/src/libstd/collections/hash/map.rs:rustc::dep_graph::prev::PreviousDepGraph::new
   431,526 ( 4.76%)  /home/njn/moz/rustN/src/librustc_metadata/decoder.rs:<rustc_metadata::schema::Entry as serialize::serialize::Decodable>::decode
  -408,336 (-4.51%)  /home/njn/moz/rustN/src/librustc_metadata/schema.rs:<rustc_metadata::schema::Entry as serialize::serialize::Decodable>::decode
   380,927 ( 4.20%)  /home/njn/moz/rustN/src/libcore/intrinsics.rs:rustc::dep_graph::prev::PreviousDepGraph::new
   379,621 ( 4.19%)  /home/njn/moz/rustN/src/libserialize/opaque.rs:<serialize::opaque::Decoder as serialize::serialize::Decoder>::read_usize
   338,776 ( 3.74%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:rustc_incremental::persist::load::load_dep_graph::{{closure}}::{{closure}}
   301,719 ( 3.33%)  /home/njn/moz/rustN/src/librustc/dep_graph/graph.rs:rustc::dep_graph::graph::DepGraph::try_mark_previous_green
   293,040 ( 3.23%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:<u32 as serialize::serialize::Encodable>::encode
   281,777 ( 3.11%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:<u32 as serialize::serialize::Encodable>::encode
   276,688 ( 3.05%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:<serialize::opaque::Decoder as serialize::serialize::Decoder>::read_usize
   245,283 ( 2.71%)  /home/njn/moz/rustN/src/libcore/num/mod.rs:rustc::dep_graph::prev::PreviousDepGraph::new
   217,059 ( 2.39%)  /home/njn/moz/rustN/src/libstd/collections/hash/map.rs:rustc::dep_graph::graph::CurrentDepGraph::intern_node
  -189,246 (-2.09%)  /home/njn/moz/rustN/src/librustc_metadata/decoder.rs:serialize::serialize::Decoder::read_struct_field
   183,414 ( 2.02%)  /home/njn/moz/rustN/src/librustc/dep_graph/graph.rs:rustc::dep_graph::graph::CurrentDepGraph::intern_node
   177,540 ( 1.96%)  /home/njn/.cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.7/lib.rs:smallvec::SmallVec<A>::push
   169,388 ( 1.87%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<u32 as serialize::serialize::Encodable>::encode
   151,618 ( 1.67%)  /home/njn/moz/rustN/src/libcore/slice/mod.rs:rustc::dep_graph::graph::DepGraph::try_mark_previous_green
   148,582 ( 1.64%)  /home/njn/moz/rustN/src/libcore/slice/mod.rs:rustc::dep_graph::graph::DepGraph::serialize
   129,970 ( 1.43%)  /home/njn/moz/rustN/src/libstd/collections/hash/table.rs:rustc_resolve::Resolver::set_binding_parent_module
   129,382 ( 1.43%)  /home/njn/moz/rustN/<::rustc::ty::codec::__impl_decoder_methods macros>:<rustc_metadata::schema::Entry as serialize::serialize::Decodable>::decode
   122,276 ( 1.35%)  /home/njn/moz/rustN/src/librustc_data_structures/fingerprint.rs:rustc_data_structures::fingerprint::Fingerprint::encode_opaque
   103,533 ( 1.14%)  /home/njn/moz/rustN/src/libcore/slice/mod.rs:<serialize::opaque::Decoder as serialize::serialize::Decoder>::read_usize
   100,119 ( 1.10%)  /home/njn/moz/rustN/src/librustc/ty/query/on_disk_cache.rs:rustc::ty::query::on_disk_cache::OnDiskCache::load_diagnostics
    99,423 ( 1.10%)  /home/njn/moz/rustN/src/libstd/collections/hash/table.rs:rustc::dep_graph::graph::CurrentDepGraph::intern_node
    93,258 ( 1.03%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:rustc_incremental::persist::load::load_dep_graph::{{closure}}::{{closure}}

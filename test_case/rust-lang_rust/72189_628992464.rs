
--------------------------------------------------------------------------------
Ir                   
--------------------------------------------------------------------------------
405,463,889 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                    file:function
--------------------------------------------------------------------------------
119,174,913 (29.39%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:serialize::serialize::Encoder::emit_enum_variant
101,718,375 (25.09%)  /home/njn/moz/rustN/src/librustc_middle/ty/query/on_disk_cache.rs:<rustc_middle::ty::query::on_disk_cache::CacheEncoder<E> as serialize::serialize::Encoder>::emit_u32
 72,955,950 (17.99%)  /home/njn/moz/rustN/src/librustc_metadata/rmeta/encoder.rs:<rustc_metadata::rmeta::encoder::EncodeContext as serialize::serialize::Encoder>::emit_u32
 49,260,540 (12.15%)  /home/njn/moz/rustN/src/libserialize/serialize.rs:serialize::serialize::Encoder::emit_seq
 47,003,165 (11.59%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<rustc_middle::ty::query::on_disk_cache::CacheEncoder<E> as serialize::serialize::Encoder>::emit_u32
 46,699,490 (11.52%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<rustc_metadata::rmeta::encoder::EncodeContext as serialize::serialize::Encoder>::emit_u32
-33,793,369 (-8.33%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<rustc_middle::ty::query::on_disk_cache::CacheEncoder<E> as serialize::serialize::SpecializedEncoder<rustc_span::span_encoding::Span>>
::specialized_encode
 31,437,956 ( 7.75%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:<rustc_metadata::rmeta::encoder::EncodeContext as serialize::serialize::Encoder>::emit_u32
-27,748,168 (-6.84%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<rustc_metadata::rmeta::encoder::EncodeContext as serialize::serialize::SpecializedEncoder<rustc_span::span_encoding::Span>>::speciali
zed_encode
 27,237,498 ( 6.72%)  /home/njn/moz/rustN/src/liballoc/vec.rs:serialize::serialize::Encoder::emit_enum_variant
 26,374,177 ( 6.50%)  /home/njn/moz/rustN/src/librustc_data_structures/sip128.rs:rustc_ast::ast::_DERIVE_rustc_data_structures_stable_hasher_HashStable_CTX_FOR_LitKind::<impl rustc_data_structures
::stable_hasher::HashStable<__CTX> for rustc_ast::ast::LitKind>::hash_stable
-25,753,232 (-6.35%)  /home/njn/moz/rustN/src/liballoc/vec.rs:<(T10,T11) as serialize::serialize::Encodable>::encode
 25,592,894 ( 6.31%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:<rustc_middle::ty::query::on_disk_cache::CacheEncoder<E> as serialize::serialize::Encoder>::emit_u32
-25,042,960 (-6.18%)  /home/njn/moz/rustN/src/librustc_middle/mir/mod.rs:<rustc_middle::mir::Operand as serialize::serialize::Encodable>::encode
 25,035,332 ( 6.17%)  /home/njn/moz/rustN/src/librustc_middle/mir/mod.rs:serialize::serialize::Encoder::emit_enum_variant
-21,562,592 (-5.32%)  /home/njn/moz/rustN/src/librustc_data_structures/sip128.rs:core::hash::impls::<impl core::hash::Hash for u64>::hash
-21,446,205 (-5.29%)  /home/njn/moz/rustN/src/libserialize/leb128.rs:<rustc_metadata::rmeta::encoder::EncodeContext as serialize::serialize::SpecializedEncoder<rustc_span::span_encoding::Span>>::specialized_encode

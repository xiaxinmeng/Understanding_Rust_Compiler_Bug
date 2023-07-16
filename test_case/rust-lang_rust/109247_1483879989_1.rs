
    0.00 :   1349543:        call   2a49c10 <<rustc_query_system::dep_graph::serialized::SerializedDepNodeIndex as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode>
    0.00 :   1349548:        mov    %eax,0x1a0(%rsp)
    0.00 :   134954f:        cmp    0x2c(%rsp),%eax
    0.00 :   1349553:        jne    1349b9a <<rustc_query_system::dep_graph::graph::DepGraphData<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>+0x12ea>
    0.00 :   1349559:        lea    0xb0(%rsp),%rdi
    0.00 :   1349561:        call   294fa00 <<thin_vec::ThinVec<rustc_errors::diagnostic::Diagnostic> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode>
    0.00 :   1349566:        mov    %rax,%rbp
    0.00 :   1349569:        mov    %rax,0x170(%rsp)
    0.00 :   1349571:        lea    0xb0(%rsp),%rdi
    0.00 :   1349579:        mov    0x28(%rdi),%r15
    0.00 :   134957d:        call   2a49b60 <<u64 as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode>

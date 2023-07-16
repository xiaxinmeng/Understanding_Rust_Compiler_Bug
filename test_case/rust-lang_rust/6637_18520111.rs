
...
#-----------
snapshot=32
#-----------
time=99635562745
mem_heap_B=1706284326
mem_heap_extra_B=93736826
mem_stacks_B=0
heap_tree=detailed
n7: 1706284326 (heap allocation functions) malloc/new/new[], --alloc-fns, etc.
 n1: 633714405 0x2E37219: rust_exchange_alloc::realloc(void*, unsigned long) (rust_exchange_alloc.cpp:32)
  n1: 633714405 0x2E2D67C: vec_reserve_shared (rust_util.h:75)
   n3: 633714405 0x1659D: vec::rustrt::vec_reserve_shared::_fd87ae5bf33686b::_07pre
    n2: 536870960 0x1375924: smallintmap::__extensions__::insert_60798::_4ecd04f78c24ce::_07pre
     n1: 536870960 0x1592486: middle::astencode::decode_side_tables::anon::expr_fn_79872
      n1: 536870960 0x50C70D: ebml::reader::docs::_29cfc8699845a3a::_07pre
       n1: 536870960 0xDA1A8F: middle::astencode::decode_inlined_item::_cd72efb8a779f80::_07pre
        n1: 536870960 0xD9FC48: middle::trans::inline::maybe_instantiate_inline::anon::expr_fn_19981
...

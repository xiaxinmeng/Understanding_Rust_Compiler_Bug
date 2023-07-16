
--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
 60 180,036,106,317    2,511,642,448    2,394,469,533   117,172,915            0
 61 181,210,964,676    2,524,188,480    2,406,285,136   117,903,344            0
95.33% (2,406,285,136B) (heap allocation functions) malloc/new/new[], --alloc-fns, etc.
->24.66% (622,355,697B) 0x2E2D219: rust_exchange_alloc::realloc(void*, unsigned long) (rust_exchange_alloc.cpp:32)
| ->24.66% (622,355,697B) 0x2E2367C: vec_reserve_shared (rust_util.h:75)
|   ->24.66% (622,355,697B) 0x1659D: vec::rustrt::vec_reserve_shared::_bcc196118a2caa62::_07pre
|     ->21.27% (536,870,960B) 0x1373874: smallintmap::__extensions__::insert_60769::_4ecd04f78c24ce::_07pre
|     | ->21.27% (536,870,960B) 0x1588B36: middle::astencode::decode_side_tables::anon::expr_fn_80017
|     | | ->21.27% (536,870,960B) 0x50B71D: ebml::reader::docs::_29cfc8699845a3a::_07pre
|     | |   ->21.27% (536,870,960B) 0xDA053F: middle::astencode::decode_inlined_item::_2fc881d8e6edb7c7::_07pre



error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Missing token range for attribute
  --> src/main.rs:43:15
   |
43 | produce_it!({ #![cfg_attr(not(FALSE), allow(unused))] 30 });
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_parse/src/parser/attr_wrapper.rs:274:22

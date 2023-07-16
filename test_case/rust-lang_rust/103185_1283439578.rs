diff
- let hi = cx.sess().source_map().next_point(remove_span).hi();
- let fmpos = cx.sess().source_map().lookup_byte_offset(hi);
+ let fmpos = cx.sess().source_map().lookup_byte_offset(remove_span.hi());

rust
error: borrowed value does not live long enough
  --> src/history.rs:65:36
   |
65 |                       .filter_map(|ref line| self.make_entry(line))
   |                                    ^^^^^^^^                      - temporary value only lives until here
   |                                    |
   |                                    does not live long enough
   |
   = note: borrowed value must be valid for the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(1511/CallSiteScope { fn_id: NodeId(566), body_id: NodeId(2401) })...

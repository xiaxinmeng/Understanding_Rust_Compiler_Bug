
rustc 1.15.0-beta.1 (d9a0f0df7 2016-12-19)
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> <anon>:2:9
  |
2 |     a = b;
  |         ^
  |
  = note: ...the reference is valid for the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(4/CallSiteScope { fn_id: NodeId(4), body_id: NodeId(22) })...
  = note: ...but the borrowed content is only valid for the anonymous lifetime #2 defined on unknown free region bounded by scope CodeExtent(4/CallSiteScope { fn_id: NodeId(4), body_id: NodeId(22) })
help: consider using an explicit lifetime parameter as shown: fn test<'a>(mut a: &'a i32, b: &'a i32)
 --> <anon>:1:1
  |
1 |   fn test(mut a: &i32, b: &i32) {
  |  _^ starting here...
2 | |     a = b;
3 | | }
  | |_^ ...ending here

error: aborting due to previous error

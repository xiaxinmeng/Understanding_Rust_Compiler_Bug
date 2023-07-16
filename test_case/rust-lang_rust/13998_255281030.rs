
error[E0308]: mismatched types
  --> src/flow/nodes/scale_render.rs:78:13
   |
78 |             f
   |             ^ expected concrete lifetime, found bound lifetime parameter 
   |
   = note: expected type `fn(&mut flow::definitions::OpCtxMut<'_>, petgraph::graph::NodeIndex)`
   = note:    found type `fn(&'a mut flow::definitions::OpCtxMut<'a>, petgraph::graph::NodeIndex) {flow::nodes::scale_render::scale_def::f}`
   = note: expected concrete lifetime is lifetime ReSkolemized(0, BrAnon(0))

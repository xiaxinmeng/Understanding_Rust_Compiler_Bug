
   Compiling panopticon-core v0.16.0 (file:///home/m4b/projects/panopticon/core)
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
  --> function.rs:92:43
   |
92 |         FunctionIter { iter: Box::new(cfg.vertices().filter_map(move |vx| cfg.vertex_label(vx))) }
   |                                           ^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the impl at 89:1...
  --> function.rs:89:1
   |
89 | / impl<'a> FunctionIter<'a> {
90 | |     /// 
91 | |     pub fn new(cfg: &'a ControlFlowGraph) -> Self {
92 | |         FunctionIter { iter: Box::new(cfg.vertices().filter_map(move |vx| cfg.vertex_label(vx))) }
93 | |     }
94 | | }
   | |_^
note: ...so that reference does not outlive borrowed content
  --> function.rs:92:39
   |
92 |         FunctionIter { iter: Box::new(cfg.vertices().filter_map(move |vx| cfg.vertex_label(vx))) }
   |                                       ^^^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that expression is assignable (expected std::boxed::Box<std::iter::Iterator<Item=&function::ControlFlowTarget> + 'static>, found std::boxed::Box<std::iter::Iterator<Item=&function::ControlFlowTarget>>)

 rust
     terminated: Cell<bool>,
     unreachable: Cell<bool>,

     /// Is this block part of a landing pad?
     is_lpad: bool,

     /// AST node-id associated with this block, if any. Used debugging purposes only.
     opt_node_id: Option<ast::NodeId>,

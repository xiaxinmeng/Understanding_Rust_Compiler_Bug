
  RsvgNode *node1 = rsvg_rust_cnode_new (...);  /* returns a Box::into_raw (Box::new (Rc::new (Node))) */
  RsvgNode *node2 = rsvg_node_ref (node1);

  assert (node1 == node2);

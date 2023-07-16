
     fn handle_reflow(&mut self, data: &Reflow) {
         // FIXME: Isolate this transmutation into a "bridge" module.
         let node2: &mut LayoutNode = unsafe {
             let mut node1: JS<Node> = JS::from_trusted_node_address(data.document_root);
             mem::transmute(&mut node1)
         };

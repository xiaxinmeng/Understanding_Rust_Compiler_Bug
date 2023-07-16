
fn root_node(&self) -> Option<GeckoLayoutNode<'ld>> {
    unsafe {
        let root = Gecko_RootNode(self.document);
        if root.is_null() {
            None         
        } else {         
            Some(GeckoLayoutNode::from_raw(root))
        }                
    }                    
}

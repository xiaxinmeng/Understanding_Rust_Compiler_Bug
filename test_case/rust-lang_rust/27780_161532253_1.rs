
fn root_node(&self) -> Option<GeckoLayoutNode<'ld>> {
    unsafe {
        Gecko_RootNode(self.document).as_ref().map(|n| GeckoLayoutNode::from_ref(n))
    }                                                                         
}

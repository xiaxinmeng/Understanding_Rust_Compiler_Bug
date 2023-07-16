
pub fn impl_item_is_final(self, node_item: &NodeItem<hir::Defaultness>) -> bool {
    node_item.item.is_final() && !self.impl_is_default(node_item.node.def_id())
}

rust
        self.with_collect_item_sig(item.id, convert_item);
// becomes
        convert_item(self.tcx, item.id);

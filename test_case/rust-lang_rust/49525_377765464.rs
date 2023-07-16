rust
        let mut items: Vec<_> = self.items().iter().map(|(&i, &l)| (i, l)).collect();
        items.sort_by_cached_key(|&(i, _)| item_sort_key(tcx, i));
        items

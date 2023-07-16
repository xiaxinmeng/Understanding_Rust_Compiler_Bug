rust
        fn visit_item(&mut self, i: &'v hir::Item) {
            if let hir::ItemKind::Trait(..) = i.node {
                let def_id = self.map.local_def_id_from_hir_id(i.hir_id);
                self.traits.push(def_id);
            }
        }

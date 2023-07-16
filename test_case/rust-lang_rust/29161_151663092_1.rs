 rust
        let def_id = source_did.unwrap_or(to_check);
        debug!("ensure_public: def_id = {:?}", def_id);
        let node_id = self.tcx.map.as_local_node_id(def_id).unwrap();

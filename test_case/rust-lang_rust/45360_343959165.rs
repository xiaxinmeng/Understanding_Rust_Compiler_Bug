rust
        StatementKind::StorageDead(local) => {
            if self.storage_dead_or_drop_error_reported.insert(local) {
                self.access_lvalue(ContextKind::StorageDead.new(location),
                                   (&Lvalue::Local(local), span),
                                   (Shallow(None), Write(WriteKind::StorageDead)),
                                   flow_state);
            }
        }

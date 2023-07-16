 rust
        if triple != self.triple {
            info!("Rejecting via crate triple: expected {} got {}", self.triple, triple);
            self.rejected_via_triple.push(CrateMismatch {
                path: libpath.clone(),
                got: triple.to_string()
            });
            return false;
        }

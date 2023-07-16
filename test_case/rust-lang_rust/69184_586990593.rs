rust
            ty::Projection(ref data) => {
                let tcx = self.tcx();
                self.add_constraints_from_trait_ref(current, data.trait_ref(tcx), variance);
            }

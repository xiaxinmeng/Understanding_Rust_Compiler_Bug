rust
match op.layout.variants {
            Variants::Single { index } => {
                let discr = match op.layout.ty.discriminant_for_variant(*self.tcx, index) {

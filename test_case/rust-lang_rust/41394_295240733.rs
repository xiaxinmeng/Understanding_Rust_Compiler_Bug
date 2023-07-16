rust
       loop {
            match self.variants[explicit_index].discr {
                ty::VariantDiscr::Relative(0) => break,
                ty::VariantDiscr::Relative(distance) => {
                    explicit_index -= distance;
                }
                ty::VariantDiscr::Explicit(expr_did) => {
                    match queries::monomorphic_const_eval::get(tcx, DUMMY_SP, expr_did) {
                        Ok(ConstVal::Integral(v)) => {
                            explicit_value = v;
                            break;
                        }
                        _ => {
                            explicit_index -= 1;
                        }
                    }
                }
            }

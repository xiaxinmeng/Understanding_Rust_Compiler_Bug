rust
    match $scrutinee {
        VariantA(a) => { .. }
        VariantB(it) match &**it {
            VariantX(x) => { .. }
            VariantY(y) => { .. }
        }
        c => { .. }
    }
    
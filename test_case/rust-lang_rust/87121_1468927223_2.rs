rust
'0: match $scrutinee {
    VariantA(a) => { .. }
    VariantB(k#placeref _5) => match deref(_5) {
        VariantX(x) => { .. }
        VariantY(y) => { .. }
        _ => continue '0,
    }
    c => { .. }
}

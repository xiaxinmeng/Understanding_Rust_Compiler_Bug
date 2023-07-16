rust
enum Scrutinee {
    VariantA(_),
    VariantB(impl Deref<Inner>),
    VariantC(_),
}

enum Inner {
    VariantX(_),
    VariantY(_),
    VariantZ(_),
}


match $scrutinee { // &Scrutinee
    VariantA(a) => { .. }
    VariantB(VariantX(x)) => { .. }
    VariantB(VariantY(y)) => { .. }
    c => { .. } 
}

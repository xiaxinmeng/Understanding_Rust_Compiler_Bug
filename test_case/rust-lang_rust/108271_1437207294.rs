rust
pub trait TraitWAssocConst<T> {
    const A: T;
}

fn main<T, B: TraitWAssocConst<T, A = { 1 }>>() {}


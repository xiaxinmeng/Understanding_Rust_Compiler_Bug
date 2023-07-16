rust
pub enum TypeCtor {
    Slice,
    Array,
}

pub struct ApplicationTy(TypeCtor);

macro_rules! ty_app {
    ($ctor:pat) => {
        ApplicationTy($ctor)
    };
}

fn _foo(ty: ApplicationTy) -> usize {
    return match ty {
        ty_app!(TypeCtor::Array) | ty_app!(TypeCtor::Slice) => 1,
    };

    // same as above, with the macro expanded
    match ty {
        crate::ApplicationTy(TypeCtor::Array)
        | crate::ApplicationTy(TypeCtor::Slice) => {}
    }
}

fn main() {}

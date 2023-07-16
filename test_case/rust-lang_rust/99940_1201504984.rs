rust
// crate `dep`
pub trait Assoc {
    type Ty;
}
impl Assoc for () {
    type Ty = ();
}

pub trait Trait {}
impl Trait for <() as Assoc>::Ty {} // err
// impl Trait for () {} // ok

// local crate
struct LocalTy;
impl dep::Trait for LocalTy {}

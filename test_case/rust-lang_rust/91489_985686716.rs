rust
#![feature(const_deref)]
#![feature(const_trait_impl)]

use std::borrow::Borrow;
use std::borrow::Cow;

pub struct VariantType {}
pub struct VariantTy {}

impl Borrow<VariantTy> for VariantType {
    fn borrow(&self) -> &VariantTy {
        unimplemented!()
    }
}

impl ToOwned for VariantTy {
    type Owned = VariantType;
    fn to_owned(&self) -> VariantType {
        unimplemented!()
    }
}

impl VariantTy {
    pub const fn as_str(&self) -> () {}
}

static _TYP: () = {
    Cow::Borrowed(&VariantTy {}).as_str();
};

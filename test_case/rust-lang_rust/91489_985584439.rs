rust
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
    pub fn as_str(&self) -> () {}
}

static _TYP: () = {
    let _ = || {
        Cow::Borrowed(&VariantTy {}).as_str();
    };
};

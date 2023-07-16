rust
#![cfg_attr(docsrs, feature(doc_cfg))]

pub struct SomeType;

pub trait SomeTrait {
    type SomeAssoc;

    fn some_method(&self);
}

#[cfg(feature = "some_feature")]
#[cfg_attr(docsrs, doc(cfg(feature = "some_feature")))]
impl SomeTrait for SomeType {
    type SomeAssoc = i32;

    fn some_method(&self) {}
}

rust
#![feature(doc_cfg)]

#[doc(cfg(feature = "sync"))]
#[doc(cfg(feature = "sync"))]
pub struct Foo;

#[doc(cfg(feature = "sync"))]
pub mod bar {
    #[doc(cfg(feature = "sync"))]
    pub struct Bar;
}

rust
#[warn(rustdoc::broken_intra_doc_links)]

/// We have some module docs here.
pub mod foo {
    //! [`Foo`]
    //!
    //! And some more module docs here.
    pub struct Foo;
}

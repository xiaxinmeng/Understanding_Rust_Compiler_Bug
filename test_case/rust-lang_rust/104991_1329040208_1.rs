rust

#[doc(
    cfg(
        feature = "abc",
    )
)]
pub mod abc {
    #[doc(
        cfg(
            feature = "a",
        )
    )]
    pub mod a {
        // ...
    }
}

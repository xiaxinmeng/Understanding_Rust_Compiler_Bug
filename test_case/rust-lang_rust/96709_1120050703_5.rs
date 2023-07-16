rust
    #[doc(hidden)]
    fn _ignore_me<'unrelated>(&self) -> &'static Self::Gat<'unrelated> {
        unimplemented!("This is a hack, not an ABI guarantee; do not call")
    }

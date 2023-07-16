rust
    fn chain(&self) -> Chain<'_> {
        Chain {
            current: Some(self),
        }
    }

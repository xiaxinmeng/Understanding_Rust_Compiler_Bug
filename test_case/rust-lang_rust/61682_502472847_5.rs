rust
        match self {
            Self::Some(a) => {
                Self::Some(transform(a))
            }
            Self::None => {
                Self::None
            }
        }

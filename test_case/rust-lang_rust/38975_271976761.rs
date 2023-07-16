
    #[inline]
    fn void_unwrap(self) -> T {
        match self {
            Ok(val) => val,
        }
    }

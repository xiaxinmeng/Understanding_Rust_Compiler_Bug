rs
    type Keys<'s>: Iterator<Item = &'s K>
    where
        K: 's,
        Self: 's;

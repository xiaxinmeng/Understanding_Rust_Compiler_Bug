rust
impl PartialEq<Evil<'c, 'd>> for Evil<'a, 'b>
    where 'a = 'c, 'b = 'd

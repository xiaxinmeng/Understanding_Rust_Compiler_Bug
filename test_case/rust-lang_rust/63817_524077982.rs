rust
    fn filter_with<'leap, Tuple: Ord, Func: Fn(&Tuple) -> (Key, Val)>(
        &'leap self,
        key_func: Func,
    ) -> filter_with::FilterWith<'leap, Key, Val, Tuple, Func>
    where
        Key: 'leap,
        Val: 'leap;

rust
    fn is_sorted_by<'a, F>(&'a self, mut compare: F) -> bool
    where
        F: FnMut(&'a T, &'a T) -> Option<Ordering>
    
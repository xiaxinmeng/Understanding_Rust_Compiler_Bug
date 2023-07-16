rust
impl Ord for Test {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_disc = mem::discriminant(self);
        let other_disc = mem::discriminant(other);

        if self_disc == other_disc {
            Ordering::Equal
        } else {
            Ord::cmp(&self_disc, &other_disc)
        }
    }
}

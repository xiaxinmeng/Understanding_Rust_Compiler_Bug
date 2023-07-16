rust
impl Ord for Test {
    fn cmp(&self, other: &Self) -> Ordering {
        if mem::discriminant(self) == mem::discriminant(other) {
            Ordering::Equal
        } else {
            match self {
                Test::A => Ordering::Less,
                Test::B => match other {
                    Test::A => Ordering::Greater,
                    _ => Ordering::Less,
                },
                Test::C => match other {
                    Test::A | Test::B => Ordering::Greater,
                    _ => Ordering::Less,
                },
                Test::D => Ordering::Greater,
            }
        }
    }
}

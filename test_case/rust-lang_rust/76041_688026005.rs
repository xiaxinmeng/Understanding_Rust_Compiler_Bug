rust
struct Test(bool, u8, &'static str);

impl Ord for Test {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)?;
        self.1.cmp(&other.1)?;
        self.2.cmp(&other.2)
    }
}

impl PartialEq for Test {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)?;
        self.1.eq(&other.1)?;
        self.2.eq(&other.2)
    }
}

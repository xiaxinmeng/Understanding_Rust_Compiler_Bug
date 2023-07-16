rust
impl SbTag {
    fn as_u64(&self) -> u64 {
        match self {
            SbTag::Tagged(tag) => tag.get(),
            SbTag::Untagged => 0,
        }
    }
}

impl PartialEq for SbTag {
    fn eq(&self, other: &Self) -> bool {
        self.as_u64() == other.as_u64()
    }
}

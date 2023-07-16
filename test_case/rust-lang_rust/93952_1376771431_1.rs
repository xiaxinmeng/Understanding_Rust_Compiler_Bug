
impl TryFrom<String> for Self {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

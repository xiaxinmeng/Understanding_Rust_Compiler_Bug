rust
pub struct TriggerICE<'a>(&'a str);
impl<'a> From<String> for TriggerICE<'a> {
    fn from(s: String) -> Self {
        TriggerICE(&s)
    }
}

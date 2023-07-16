rust
use std::borrow::Borrow;

struct Label(String);

impl Borrow<str> for Label {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}
impl<'a> From<&'a str> for Label {
    fn from(s: &'a str) -> Self {
        Label(s.to_string())
    }
}

fn get<Q: ?Sized>(k: &Q)
where
    Label: Borrow<Q>,
{
}

fn iter(f: impl FnMut(Label)) {}

fn main() {
    get(&"k".into());
    iter(|k: Label| ());
}

rust
#[derive(Clone)]
struct X;

impl<'a> From<&'a X> for Cow<'a, X> {
    fn from(x: &'a X) -> Self {
         Cow::Borrowed(x)
    }
}

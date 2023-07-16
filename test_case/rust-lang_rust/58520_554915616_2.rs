rust
#[derive(Debug)]
struct B(Option<Box<dyn Error + 'static>>);

impl Error for B {
    fn chain(&self) -> Chain<'_> {
        Chain {
            current: Some(self),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.as_ref().map(|e| e.as_ref())
    }
}

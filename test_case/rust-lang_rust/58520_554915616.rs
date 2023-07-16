rust
pub trait Error: Debug + Display {
    fn sources(&self) -> Chain<'_> {
        Chain {
            current: self.source(),
        }
    }

    fn chain(&self) -> Chain<'_> {
        Chain { current: None }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

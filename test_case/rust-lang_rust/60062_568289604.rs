
impl<G> Iterator for G where G: PinCell< Generator<Return = ()> > {
    type Item = G::Content::Yield;
    
    fn next(&mut self) -> Option<Self::Item> {
        match (self.borrow_pin_mut().unwrap()).resume() {
            GeneratorState::Yielded(item) => Some(item),
            GeneratorState::Complete(_) => None,
        }
    }
}


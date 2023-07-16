rust
pub fn with<F, O>(&self, f: F) -> Result<O, PoisonError<F>> where F: FnOnce(&mut T) -> O

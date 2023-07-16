rust
use std::fmt::{Debug, Formatter};

pub struct Irrelevant<Irrelevant> {
    irrelevant: Irrelevant,
}

impl<Irrelevant: Debug> Debug for Irrelevant<Irrelevant> {
    fn fmt(&self, f: &mut Formatter) -> ::core::fmt::Result {
        match self {
            Irrelevant { irrelevant } => {
                f.debug_struct("Irrelevant")
                    .field("irrelevant", irrelevant)
                    .finish()
            }
        }
    }
}

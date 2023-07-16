rust
use std::borrow::ToOwned;

struct Element<'a> {
    arr: CowVec<'a, Element<'a>>,
}

impl Clone for Element<'_> {
    fn clone(&self) -> Self { todo!() }
}

enum CowVec<'a, B> 
where
    B: 'a + Clone, 
 {
    Borrowed(&'a [B]),
    Owned(<[B] as ToOwned>::Owned),
}

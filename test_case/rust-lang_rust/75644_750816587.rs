rust
// I've written
fn unfold<St, F>(init: St, f: F) -> Self
where
    F: FnMut(&mut St) -> Self::Item;

// How it probably should be to be analogous to Iterator::fold
fn unfold<St, F>(init: St, f: F) -> Self
where
    F: FnMut(St) -> (Self::Item, St);

rust
fn any(&mut self, f: impl FnMut(Self::Item) -> bool) -> bool {
    IteratorSpecializations::any(self, f)
}

/* Default IteratorSpecializations::any uses try_fold */
/* libcore/alloc iterators marked as non-segmented use simple loop? */


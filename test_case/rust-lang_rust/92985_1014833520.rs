rust
    fn next<'a>(&'a mut self) -> Option<I::Item<'a>> {
        while let Some(mut item) = self.iter.next() {
            if (self.predicate)(&item) {
                // SAFETY: We turn a lifetime of "`'x`" where `'a <: 'x` into a `'a`.
                // This is safe since we immediately return this item.
                // This workaround isn't needed under Polonius.
                let item = unsafe {
                    let ptr = &mut item as *mut Self::Item<'_>;
                    let ptr = std::mem::transmute::<*mut Self::Item<'_>, *mut Self::Item<'a>>(ptr);
                    let res = ptr.read();
                    core::mem::forget(item);
                    res
                };
                return Some(item);
            }
        }
        return None;
    }

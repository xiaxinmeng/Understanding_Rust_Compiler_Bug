rust
// cleaner?
struct SubtreeHelper<'r, T> where Self: 'r {
    root: &'r mut T,
}

impl<'r, 's, P: Borrow<str> + Ord, O: Borrow<str> + Ord, T: PatternTypes> SubtreeHelper<'r, Parser<'s, P, O, T>> where Self: 'r {
    fn start(value: &'r mut Parser<'s, P, O, T>) -> Self {
        value.consts.protos.push(Default::default());
        Self {
            root: value,
        }
    }

    fn commit(mut self) -> usize {
        let mut self_ = ManuallyDrop::new(self);
        let proto = self_.root.consts.protos.pop().unwrap();
        let id = self_.root.closed_subtrees.next().unwrap();
        self_.root.consts.protos.insert(id, proto);
        id
    }
}

impl<'r, T> std::ops::Deref for SubtreeHelper<'r, T> where Self: 'r {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.root
    }
}

impl<'r, T> std::ops::DerefMut for SubtreeHelper<'r, T> where Self: 'r {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.root
    }
}

impl<'r, 's, P: Borrow<str> + Ord, O: Borrow<str> + Ord, T: PatternTypes> Drop for SubtreeHelper<'r, Parser<'s, P, O, T>> where Self: 'r {
    fn drop(&mut self) {
        // remove "partial" proto
        self.root.consts.protos.pop().expect("SubtreeHelper");
    }
}

rust
impl<I: IntoIterator, A: Allocator + Clone> SpecExtend<I> for LinkedList<I::Item, A> {
    default fn spec_extend(&mut self, iter: I) {
        iter.into_iter().for_each(move |elt| self.push_back(elt));
    }
}

impl<T, A: Allocator + Clone> SpecExtend<LinkedList<T, A>> for LinkedList<T, A> {
    fn spec_extend(&mut self, ref mut other: LinkedList<T, A>) {
        self.append(other);
    }
}

rust
impl<T: Ord> BinaryHeap<T> {
    fn find(&self, item: &T) -> Option<usize> {
        self.find_recursive(item, 0)
    }

    fn find_recursive(&self, item: &T, pos: usize) -> Option<usize> {
        if pos >= self.data.len() {
            None
        } else {
            match self.data[pos].cmp(item) {
                // If `self.data[pos] == item`, return the item's position.
                Ordering::Equal => Some(pos),
                // If `self.data[pos] < item`, the item cannot be in either of
                // the child branches (because this is a max heap).
                Ordering::Less => None,
                // If `self.data[pos] > item`, we need to search both child branches.
                Ordering::Greater => {
                    let left_child = 2 * pos + 1;
                    let right_child = 2 * pos + 2;
                    self.find_recursive(item, left_child)
                        .or_else(|| self.find_recursive(item, right_child))
                }
            }
        }
    }
}

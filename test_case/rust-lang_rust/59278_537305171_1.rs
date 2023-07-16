
    #[unstable(feature = "binary_heap_into_ordered_iter", issue = "59278")]
    pub fn into_ordered_iter(self) -> IntoOrderedIter<T> {
        IntoOrderedIter {
            inner: self,
        }
    }

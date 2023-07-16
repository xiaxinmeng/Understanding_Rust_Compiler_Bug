
    #[inline]
    #[unstable(feature = "binary_heap_drain_ordered", issue = "59278")]
    pub fn drain_ordered(&mut self) -> DrainOrdered<'_, T> {
        DrainOrdered {
            inner: self,
        }
    }

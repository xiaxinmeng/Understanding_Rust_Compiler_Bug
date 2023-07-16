rust
    fn clamp<T>(mut self, low: &T, high: &T) -> Self
        where T: ?Sized + ToOwned<Owned=Self> + Ord, Self : Borrow<T>
    {
        assert!(low <= high);
        if self.borrow() < &low {
            low.clone_into(&mut self);
        } else if self.borrow() >= &high {
            high.clone_into(&mut self);
        }
        self
    }

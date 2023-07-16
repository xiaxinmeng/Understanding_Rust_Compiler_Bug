rust
pub fn defer<F: FnOnce()>(f: F) -> OnDrop<F> {
    OnDrop(ManuallyDrop::new(f))
}

pub struct OnDrop<F: FnOnce()>(ManuallyDrop<F>);

impl<F: FnOnce()> OnDrop<F> {
    pub fn disable(self) {
        // Prevent `self`'s drop from running and taking/calling `self.0`
        let mut this = ManuallyDrop::new(self);
    
        // Safety:
        // `self.0` is only taken on drop and we prevented it from running.
        unsafe { ManuallyDrop::drop(&mut this.0) };
    }
}

impl<F: FnOnce()> Drop for OnDrop<F> {
    fn drop(&mut self) {
        // Safety:
        // This is the only place that takes `self.0` (except `disable` which is
        // mutually exclusive with this function);
        let f = unsafe { ManuallyDrop::take(&mut self.0) };
        f();
    }
}

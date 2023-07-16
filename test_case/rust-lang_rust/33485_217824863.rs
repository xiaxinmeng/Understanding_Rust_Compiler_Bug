 rust
impl {
    fn f(&self) {
        m!(); // If `m`'s expansion uses `self`, we wouldn't be able to refactor it ...
        let g = |this: &Self| { // ... into this closure
            m!();
        };
        g(self);
    }
}

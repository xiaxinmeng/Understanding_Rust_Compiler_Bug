 Rust
impl Spam {
    fn get_eggs(&self, cooked: bool) -> &Eggs {
        if cooked {
            self.eggs.set(Eggs::cook());
        }
        self.eggs.get_with(|| Eggs::raw())
    }
}

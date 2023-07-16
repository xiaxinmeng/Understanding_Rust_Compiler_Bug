rust
impl S {
    fn update(&mut self) {
        let iter = self.0.iter_mut().map(|p| -> &mut (dyn std::fmt::Debug) { &mut **p });
        use_it(iter);
    }
}

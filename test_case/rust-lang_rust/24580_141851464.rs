
impl Sides {
    // Execute all the code paths to shut up warnings.
    // FIX: https://github.com/rust-lang/rust/issues/24580
    #[allow(dead_code)]
    fn _dead_code(&mut self) {
        self.is_all();
        self.is_empty();
        self.bits();
        self.intersects(*self);
        self.remove(SIDE_RIGHT);
        self.toggle(SIDE_RIGHT);
        Sides::from_bits(0b00000000);
        Sides::from_bits_truncate(0b00000000);
    }
}

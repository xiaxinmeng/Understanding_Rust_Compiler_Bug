
impl CharClass {
    pub fn new(ranges: ~[(char, char)]);
    pub fn negate(&self) -> self;
    // ...
}

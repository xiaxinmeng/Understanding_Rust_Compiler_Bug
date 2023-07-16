
struct S {
    x: u8,
    y: u8,

    fn inherent_method() -> u8 { self.x + self.y }

    as Trait {
         fn trait_method() -> u8 { self.x - self.y }
    }
}

 rust
struct Foo { ... }

impl Ord for Foo {
    fn cmp(&self, other: &Foo) -> Ordering {
        // ... lots of stuff ...
    }
}
impl PartialOrd for Foo {
     fn partial_cmp(&self, other: &Foo) -> Option<Ordering> { Some(self.cmp(other)) }
     fn lt(&self, other: &Foo) -> bool { /* supafast */ }
}

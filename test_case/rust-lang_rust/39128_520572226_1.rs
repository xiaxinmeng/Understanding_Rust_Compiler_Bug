
struct BoxedTrait(Box<dyn Trait>);

impl PartialEq for BoxedTrait {
   fn eq(&self, other: &Self) -> bool {
    <Box<dyn Trait> as PartialEq>::eq(&self.0, &other.0)
  }
}

#[derive(PartialEq)
struct Foo(BoxedTrait);

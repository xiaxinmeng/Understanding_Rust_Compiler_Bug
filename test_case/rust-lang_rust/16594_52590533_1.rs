
struct MutThing;

impl MutThing {
  pub fn mut_thing(&mut self, _: ||) {}
}

struct Foo {
    x: MutThing,
    y: int,
}

impl Foo {
    pub fn foo(&mut self) {
        let y = &mut self.y;
        self.x.mut_thing(|| {
          *y = 1;
        });
    }
}

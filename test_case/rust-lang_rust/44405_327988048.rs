rust
use std::ops::Index;

struct Test;
struct Container(Test);

impl Test {
    fn test(&mut self) {
    }
}

impl<'a> Index<&'a bool> for Container {
    type Output = Test;

    fn index(&self, index: &'a bool) -> &Test {
        &self.0
    }

}


fn main() {
    let mut container = Container(Test);
    let mut val = true;
    container[&mut val].test();
}

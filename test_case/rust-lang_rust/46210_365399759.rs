rust
use std::ops::Index;

struct Foo;
struct Bar {}

trait Test {
    type Assoc;
}

impl Test for Bar {
    type Assoc = usize;
}

impl From<Foo> for Vec<<Bar as Test>::Assoc> {
    fn from(data: Foo) -> Self {
        Vec::new()
    }
}

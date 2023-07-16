rust
use std::fmt::{Display, self};

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

fn main() {
    let instance = MyStruct;

    assert!(false, "oh no, '{}' was unexpected", instance);
}

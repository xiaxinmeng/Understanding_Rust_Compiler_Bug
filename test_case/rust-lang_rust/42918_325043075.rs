rust
use std::collections::HashSet;

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
enum MyEnum {
    E0,

    E1,

    E2,
    E3,
    E4,

    E5,
    E6,
    E7,
}


fn main() {
    let s: HashSet<_> = [MyEnum::E4, MyEnum::E1].iter().cloned().collect();

    println!("Expect only E1+E4 got {:?}", s);
}

 rust
#![feature(macro_rules)]

extern crate collections;

macro_rules! map {
    ($($k: expr => $v: expr),*) => {{
        use std::default::Default;

        let mut _thing = Default::default();
        $( _thing.insert($k, $v); )* //~ ERROR the type of this value must be known in this context
        _thing
    }}
}

pub fn main() {
    use collections::treemap::TreeMap;

    let x: TreeMap<int, int> = map!(1 => 2);
}

rust
trait IAmATrait {
    type Item;
}

impl IAmATrait for () {
    type Item = _;
}

fn main() {}

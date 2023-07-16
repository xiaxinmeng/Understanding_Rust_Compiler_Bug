rust
trait IterExtensions {}

impl<T> IterExtensions for Iterator<Item = T> 
    where Self::Item: PartialEq {}

fn main() {}

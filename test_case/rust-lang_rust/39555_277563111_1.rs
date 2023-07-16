rust
use std::slice::Iter;

pub struct Foo {
    children: Vec<Foo>,
}

impl Foo {
    pub fn iter_all_children<'a>(&'a self) -> Box<Iterator<Item=&'a Foo> + 'a> {
        Box::new(self.children.iter().flat_map(|x| x.iter_all_children())) as Box<Iterator<Item=&'a Foo> + 'a>
    }
}

fn main() {
    let foo = Foo { children: Vec::new() };
    let v: Vec<_> = foo.iter_all_children().collect();
}

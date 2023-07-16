
struct Foo {
    v: Vec<u8>
}

impl Foo {
    fn bar1(&self) -> impl Iterator<Item=u8> + '_ {
        self.v.iter().cloned()
    }

    fn bar2(&self) -> std::iter::Cloned<std::slice::Iter<u8>> {
        self.v.iter().cloned()
    }

    fn bar3<'a>(&'a self) -> impl Iterator<Item=u8> + 'a {
        self.v.iter().cloned()
    }
}

fn main() {}

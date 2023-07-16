
#[derive(PartialEq, Eq)]
pub enum Thing { Foo(bool), Bar(Vec<()>) }

impl Thing {
    pub const FOO: Thing = Thing::Foo(true);
    pub const BAR: Thing = Self::Foo(true);
}

fn main() {}

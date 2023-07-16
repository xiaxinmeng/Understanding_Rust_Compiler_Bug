rust
trait MyTrait {
    type Assoc;
    fn do_sth(arg: Self::Assoc);
}

struct Foo;

impl MyTrait for Foo {
    type Assoc = u32;
    
    fn do_sth(_: u32) {}
}

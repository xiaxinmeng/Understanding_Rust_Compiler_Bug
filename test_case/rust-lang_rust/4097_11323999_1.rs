
extern mod foo;
use foo::Foo;

fn main() {
    assert 42 == Foo::foo();
}

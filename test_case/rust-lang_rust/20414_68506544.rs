
trait Foo {
    fn foo_fn(self);
}

struct NoData;

fn any_fn(val: NoData) where NoData: Foo {
    val.foo_fn();
}

fn main() { }


struct Foo(&'static [&'static [uint]]);

static works: Foo = Foo([&'static [0, 1, 2, 3]]);
// static fails: Foo = Foo([[0, 1, 2, 3]]);

fn main() {
}

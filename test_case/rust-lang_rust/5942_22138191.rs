 rust
trait Foo { fn foo(); }
impl Foo for int { fn foo() { } }

static x: int = 5;
static y: int = 6;
static z: int = 7;

fn main() {
    let foo = (if some_undecidable_condition() { &x } else { &y }) as &Foo;
}

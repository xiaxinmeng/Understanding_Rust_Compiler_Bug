 rust
trait Foo {}

trait Bar1
{
    fn bar(&self) {}
}

trait Bar2
{
    fn bar(&self) {}
}

struct S1;
struct S2;

impl Bar1 for S1 {}
impl Foo for S2 {}
impl<T: Foo> Bar2 for T {}

fn main()
{
    (S1).bar();
}

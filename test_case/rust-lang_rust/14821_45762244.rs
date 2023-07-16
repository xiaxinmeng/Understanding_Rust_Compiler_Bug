
trait SomeTrait{}
struct Meow;
impl SomeTrait for Meow{}

struct Foo<'a> (
  &'a SomeTrait,
  &'a SomeTrait
);
impl<'a> Foo<'a> {
  pub fn new<'b>(x:&'b SomeTrait, y: &'b SomeTrait) -> Foo<'b> {
    Foo ( x, y )
  }
}
fn main() {
  let r = Meow;
  let s = Meow;
  let q = Foo::new(&r as &SomeTrait, &s as &SomeTrait);
}

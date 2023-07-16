rust
trait Foo<'_> { }
impl Foo<'_> for &u32 { }

fn bar<'a>(data: &'a u32) {
  let x: impl Foo<'a> = data;
}

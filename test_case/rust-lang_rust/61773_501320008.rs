rust
trait Trait<'a, 'b> { }
impl<T> Trait<'_, '_> for T { }


fn bar<'a>(data0: &'a u32, data1: &'b u32) {
  let x: impl Trait<'_, '_> = (data0, data1);
  force_equal(x);
}

fn force_equal<'a>(t: impl Trait<'a, 'a>) { }

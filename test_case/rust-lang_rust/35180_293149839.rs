rust
trait R<'a, 's: 'a> {}

impl<'a, 's: 'a> R<'a, 's> for u32 {}

struct S<X: for<'a> R<'a, 'static>>(X);

fn f(x: S<u32>) {}

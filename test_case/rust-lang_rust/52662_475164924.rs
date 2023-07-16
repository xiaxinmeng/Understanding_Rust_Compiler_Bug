rust
trait Foo<'a> = FnOnce<(&'a u8,)> where <Self as FnOnce<(&'a u8,)>>::Output: Future<Output = u8> + 'a;

fn foo<F: for<'a> Foo<'a>>(f: F)

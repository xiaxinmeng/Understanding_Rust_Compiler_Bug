
#![feature(old_impl_check)]

trait Foo { fn foo(&self, x: &Self); }
trait Bar<A: Foo> { fn bar(&self) -> A; }

struct Wrap<T>(T);

#[old_impl_check]
impl<A, B: Bar<A>> Wrap<B> {
        fn test(&self, x: &A) {
                    (*self).bar().foo(x);
                        }
}

fn main() {}


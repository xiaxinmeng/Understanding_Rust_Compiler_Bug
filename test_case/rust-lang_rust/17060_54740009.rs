
#![feature(unboxed_closures)]

fn main(){
    struct Foo<'a, T> {
        fun:  Box<Fn<((), ), ((), T)> + 'a>
    }

    fn bar<'a, T> (h: Foo<'a, T>) -> Foo<'a, T>{
        Foo {
            fun: box |&: _: ()| {
                h.fun.call(((), ))
            }
        }
    }

    bar(Foo {
            fun: box |&: _: ()| {
                ((), ())
            }
        }
    );
}

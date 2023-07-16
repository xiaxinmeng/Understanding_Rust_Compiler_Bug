rust
#![feature(unboxed_closures)]

trait Foo<T> {
    fn foo(&mut self) -> T;
}
impl<T> Foo<T> for Option<T> {
    fn foo(&mut self) -> T {
        self.take().unwrap()
    }
}

trait Bar<T> {
    fn bar(&mut self) -> T;
}
impl<S, T> Bar<T> for S {
    fn bar(&mut self) -> T {
        unimplemented!()
    }
}

fn transmute<S, T>(x: S) -> T {
    let x: &mut (Foo<S>) = &mut Some(x);
    step_one::<fn() -> _, T>(x)
}

fn step_one<F: FnOnce<()>, T>(x: &mut F::Output) -> T {
    let x: &mut Bar<T> = x; // casts &mut dyn Foo<S> to &mut dyn Bar<T>
                                // compiler is happy since `F::Output: Sized`
                                // on the other hand, the compiler later implements the actual
                                // cast as a no-op since the reference `x` already contains a vtable pointer

    x.bar() // effectively calls x.foo() from the vtable
}

static X: usize = 12345;

fn main() {
    let x = transmute::<&'static usize, &'static [usize; 1000000 as _]>(&X);
    assert_eq!(x[0], 12345); // works, nice!

    // let’s keep reading the rest of static memory and whatever follows:
    println!("{:?}", &x[1..]);

    // eventually segfaults
}

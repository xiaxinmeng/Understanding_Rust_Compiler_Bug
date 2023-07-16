rust
trait Foo<'a> {
    type Value;
}

impl<'a> Foo<'a> for () {
    type Value = ();
}

trait Bar<F> {
    fn bar(cb: F);
}

impl<O, F> Bar<F> for O
where
    O: for<'a> Foo<'a>,
    F: for<'b> Fn(<O as Foo<'b>>::Value),
{
    fn bar(cb: F) {
        let _: Box<dyn Fn()> = Box::new(|| cb(unsafe { std::mem::transmute_copy::<_, _>(&()) }));
    }
}

fn main() {
    <()>::bar(|_| ());
}

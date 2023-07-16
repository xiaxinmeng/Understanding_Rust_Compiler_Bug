rust
trait Bar<T> {
    type Assoc;
}
impl<'a> Bar<&'a ()> for () {
    type Assoc = Box<Send>;
}

fn oops<C>()
where
    for<'a> C: Bar<&'a ()>,
    for<'a> <C as Bar<&'a ()>>::Assoc: Send,
{
}

fn main() {
    oops::<()>();
}

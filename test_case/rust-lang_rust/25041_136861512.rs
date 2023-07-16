
trait MyFn<In> {
    type Out;
}

trait Foo {}

impl<F, A> Foo for F where F: MyFn<A, Out=()> {}

fn main() {}


trait Foo { }

struct NoData<T>;

fn any_fn<T>() where NoData<T>: Foo {
    let val = NoData::<T>;
}

fn main() { }

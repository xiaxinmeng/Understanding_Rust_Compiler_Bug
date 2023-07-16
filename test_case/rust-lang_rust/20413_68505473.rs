
trait Foo { }

struct NoData<T>;

impl<T> NoData<T> where NoData<T>: Foo {
    fn any_fn() {
        let val: NoData<T> = NoData;
    }
}

fn main() { }

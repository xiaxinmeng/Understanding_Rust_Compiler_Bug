
struct SomeItem<T>;

trait Foo {
    type Bar;
}

fn my_func<U: Foo>() {
    SomeItem<U::Bar>;
}

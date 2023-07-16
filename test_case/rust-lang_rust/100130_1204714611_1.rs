rust
struct Foo<T>(T);

fn main() {
    || {
        if false {
            return Foo(0);
        }

        Foo::<i32>(())
    };
}

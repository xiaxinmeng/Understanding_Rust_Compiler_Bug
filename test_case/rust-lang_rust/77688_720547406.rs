rust
fn use_foo(x: Foo) {
    ...
}

fn adapt_bar<const F: fn(Foo)>(bar: Bar) {
    F(bar.into_foo());
}

fn get_adapted_function_ptr<const F: fn(Foo)>() -> fn(Bar) {
    adapt_bar::<F>
}

fn main() {
    let ... = get_adapted_function_ptr::<use_foo>();
}

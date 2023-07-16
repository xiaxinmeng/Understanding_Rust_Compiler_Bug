rust
fn use_foo(x: Foo) {
    ...
}

fn adapt_bar<F: FnOnce(Foo) + Default>(bar: Bar) {
    let f = F::default();
    f(bar.into_foo());
}

fn get_adapted_function_ptr<F: FnOnce(Foo) + Default>(inner: F) -> fn(Bar) {
    adapt_bar::<F>
}

fn main() {
    let ... = get_adapted_function_ptr(use_foo);
}

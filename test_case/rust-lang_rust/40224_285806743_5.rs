rust
fn foo<T: Debug>(t: T);

fn ok4() {
    return;
    let _x = Some(return); // _x has type Option<?T> where ?T is unconstrained
    foo(_x); // (Option<?T>: Debug) only if (?T: Debug), `?T: Debug` can never be solved
}

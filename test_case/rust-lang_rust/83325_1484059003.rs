
fn my_function<'a>(x: &'a str) -> &'a str {
    &x[0..1]
}


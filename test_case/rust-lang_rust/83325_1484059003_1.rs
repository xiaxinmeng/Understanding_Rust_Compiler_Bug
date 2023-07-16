 
fn my_function<'a>(x: &'static str, y: &'a str) -> &'a str {
    if x == "foo" {
        &y[0..1]
    } else {
        y
    }
}

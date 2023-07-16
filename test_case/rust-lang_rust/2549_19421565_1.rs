
fn bar() {
    do foo |callback| {
        callback();
        callback();
    }
}


fn qux(items: ~[u32]) {}

fn quux() {
    qux(items); // must be compile-time error
}

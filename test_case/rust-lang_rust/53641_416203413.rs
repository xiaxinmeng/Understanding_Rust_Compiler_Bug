rust
struct NoCopy;

fn consume(_: NoCopy) {}

fn my_example(foos: Vec<NoCopy>) {
    let mut foos_iter = foos.into_iter();
    for foo in foos_iter.by_ref().take(6) {
        // 6 foos by value, coming right up!
        consume(foo)
    }
    for foo in foos_iter {
        // the remaining foos by value
        consume(foo)
    }
}

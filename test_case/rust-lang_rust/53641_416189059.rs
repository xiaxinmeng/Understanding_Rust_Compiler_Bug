
// no copy, no clone!
struct Foo { .. };

fn my_example(foos: Vec<Foo>) {
    let foos_iter = foos.into_iter();
    let take = foos_iter.take(6);
    while let Some(foo) = take.next() {
        .. // 6 foos by value, coming right up!
    }
    let rest = take.into_inner();
    for foo in rest {
        .. // the remaining foos by value
    }
}

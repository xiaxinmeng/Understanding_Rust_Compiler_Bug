rust
// Construct a closure, delaying `do_something_synchronous()`:
task.push(|| {
    let data = do_something_synchronous();
    StatusUpdate { data }
});

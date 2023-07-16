rust
// Execute a block, immediately running `do_something_synchronous()`:
task.push({
    let data = do_something_synchronous();
    StatusUpdate { data }
});

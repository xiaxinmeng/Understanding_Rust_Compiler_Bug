
error[E0382]: use of moved value: `runtest`
   --> library/test/src/lib.rs:475:17
    |
473 |             if cfg.spawn(runtest).is_err() {
    |                          ------- value moved here
474 |                 // If we can't spawn a new thread, just run the test synchronously.
475 |                 runtest();
    |                 ^^^^^^^ value used here after move
    |
note: closure cannot be invoked more than once because it moves the variable `desc` out of its environment
   --> library/test/src/lib.rs:451:17
    |
451 |                 desc,
    |                 ^^^^

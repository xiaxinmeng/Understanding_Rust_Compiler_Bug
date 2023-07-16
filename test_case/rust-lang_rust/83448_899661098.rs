
error[E0597]: `fh` does not live long enough
   --> src/run.rs:297:42
    |
297 |     let fh = match std::str::from_utf8(&*fh) {
    |                    ----------------------^^-
    |                    |                     |
    |                    |                     borrowed value does not live long enough
    |                    argument requires that `fh` is borrowed for `'static`
...
426 | }
    | - `fh` dropped here while still borrowed
error: aborting due to previous error
For more information about this error, try `rustc --explain E0597`.

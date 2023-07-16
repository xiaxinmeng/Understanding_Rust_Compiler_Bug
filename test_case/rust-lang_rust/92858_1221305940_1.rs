
error[E0382]: use of moved value: `cs`
  --> src/calls/mod.rs:61:26
   |
49 |         let mut cs = CallStack::new();
   |             ------ move occurs because `cs` has type `CallStack`, which does not implement the `Copy` trait
...
55 |                 cs = CallStack::new();
   |                 -- this reinitialization might get skipped
...
61 |                 css.push(cs);
   |                          ^^ value moved here, in previous iteration of loop

For more information about this error, try `rustc --explain E0382`.
error: could not compile `calltrace` due to previous error




error[E0631]: type mismatch in closure arguments
   --> src/bin/conserve.rs:435:70
    |
425 |                 let observer = |event| match event {
    |                                ------- found signature defined here
...
435 |                 let stats = restore(&archive, destination, &options, observer)?;
    |                             -------                                  ^^^^^^^^ expected due to this
    |                             |
    |                             required by a bound introduced by this call
    |
    = note: expected closure signature `for<'a, 'b> fn(&'a conserve::restore::Event<'b>) -> _`
               found closure signature `fn(conserve::restore::Event<'_>) -> _`
note: required by a bound in `conserve::restore`
   --> /Users/mbp/src/conserve/src/restore.rs:79:8
    |
79  |     O: Fn(&Event),
    |        ^^^^^^^^^^ required by this bound in `conserve::restore`

For more information about this error, try `rustc --explain E0631`.
error: could not compile `conserve` due to previous error

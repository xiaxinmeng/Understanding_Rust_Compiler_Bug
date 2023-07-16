plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `tr` in this scope
    --> src/librustdoc/clean/mod.rs:1563:56
     |
1563 | ...                   bound_predicate.rebind(tr.trait_ref)
     |                                              ^^ help: a local variable with a similar name exists: `ty`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustdoc`

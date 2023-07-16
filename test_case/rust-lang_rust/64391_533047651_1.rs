rust
$ cargo test -p offst-relay
   Compiling offst-relay v0.1.0 (/home/real/projects/d/offst/components/relay)
error[E0597]: `access_control` does not live long enough
   --> components/relay/src/client/client_listener.rs:512:17
    |
510 | /             inner_client_listener(
511 | |                 connector,
512 | |                 &mut access_control,
    | |                 ^^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
513 | |                 &mut incoming_access_control,
...   |
519 | |                 Some(event_sender)
520 | |             ).await
    | |_____________- a temporary with access to the borrow is created here ...
521 |           }
    |           -
    |           |
    |           `access_control` dropped here while still borrowed
    |           ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `impl core::future::future::Future`
    |
    = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
error: Could not compile `offst-relay`.

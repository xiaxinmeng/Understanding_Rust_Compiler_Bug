
error[E0597]: `config` does not live long enough
   --> tokio-postgres/src/lib.rs:171:5
    |
171 |     config.connect(tls).await
    |     ^^^^^^-------------
    |     |
    |     borrowed value does not live long enough
    |     a temporary with access to the borrow is created here ...
172 | }
    | -
    | |
    | `config` dropped here while still borrowed
    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `impl core::future::future::Future`
    |
    = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.

error: aborting due to previous error


 error[E0004]: non-exhaustive patterns: type `()` is non-empty
    --> src/libcore/num/mod.rs:5046:15
     |
5046 |         match never {}
     |               ^^^^^
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `()`

error: aborting due to previous error

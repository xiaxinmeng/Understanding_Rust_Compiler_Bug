rust
error[E0308]: mismatched types
   --> src\wrapper.rs:535:11
    |
533 |           do_fn(||{
    |  _______________-
534 | |             let result=arg+1;
535 | |         },|| {});
    | |         - ^^^^^ expected closure, found a different closure
    | |_________|
    |           the expected closure
    |
    = note: expected closure `[closure@src\wrapper.rs:533:15: 535:10 arg:_]`
               found closure `[closure@src\wrapper.rs:535:11: 535:16]`
    = note: no two closures, even if identical, have the same type
    = help: consider boxing your closure and/or using it as a trait object


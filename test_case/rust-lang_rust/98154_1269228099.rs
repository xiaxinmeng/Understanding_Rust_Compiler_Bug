plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0308]: mismatched types
   --> library/std/src/io/util/tests.rs:102:18
    |
102 |     Read::by_ref(e).read_buf(buf.unfilled()).unwrap();
    |     |            |
    |     |            |
    |     |            expected `&mut _`, found struct `util::Empty`
    |     |            help: consider mutably borrowing here: `&mut e`
    |
    = note: expected mutable reference `&mut _`
                          found struct `util::Empty`
note: associated function defined here

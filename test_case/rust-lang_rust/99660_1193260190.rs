plain
    Checking regex-automata v0.1.10
error[E0061]: this enum variant takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_parse_format/src/tests.rs:184:23
    |
184 |             position: ArgumentIs(1),
    |                       ^^^^^^^^^^--- an argument of type `Option<InnerSpan>` is missing
note: tuple variant defined here
   --> compiler/rustc_parse_format/src/lib.rs:108:5
    |
    |
108 |     ArgumentIs(usize, Option<InnerSpan>),
help: provide the argument
    |
    |
184 |             position: ArgumentIs(1, /* Option<InnerSpan> */),

    Checking parking_lot v0.11.2
For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_parse_format` due to previous error

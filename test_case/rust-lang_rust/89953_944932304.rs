plain
    | |_- in this expansion of `$crate::panic!` (#2)
...
589 | / macro_rules! unreachable {
590 | |     () => ({
591 | |         $crate::panic!("internal error: entered unreachable code")
    | |         |
    | |         in this macro invocation (#2)
    | |         in this macro invocation (#3)
592 | |     });
592 | |     });
...   |
598 | |     });
599 | | }
    | |_- in this expansion of `unreachable!` (#1)
   ::: library/core/tests/option.rs:388:21
    |
388 |               None => unreachable!(),
    |                       -------------- in this macro invocation (#1)

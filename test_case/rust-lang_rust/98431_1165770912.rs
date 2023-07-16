plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: `&str` is not an iterator
   --> compiler/rustc_typeck/src/check/pat.rs:698:28
    |
698 |                 ident_kind.map(|thing| (
    |                            ^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `&str: Iterator`
            which is required by `&mut &str: Iterator`
            `str: Iterator`
            which is required by `&mut str: Iterator`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to previous error

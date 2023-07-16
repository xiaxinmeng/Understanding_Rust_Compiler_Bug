
error[E0106]: missing lifetime specifier
   --> futures-io/src/lib.rs:545:32
    |
545 |                 -> Poll<Result<&[u8]>>
    |                                ^ expected lifetime parameter
...
557 |         delegate_async_buf_read_to_stdio!();
    |         ------------------------------------ in this macro invocation
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of argument 2's 2 lifetimes it is borrowed from

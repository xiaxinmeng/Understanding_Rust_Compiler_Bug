
   Compiling fmt_macros v0.0.0 (file:///checkout/src/libfmt_macros)
error: no rules expected the token `static`
   --> /checkout/src/liblog/lib.rs:215:5
    |
215 | static LOCAL_LOGGER: RefCell<Option<Box<Logger + Send>>> = {
    | ^^^^^^

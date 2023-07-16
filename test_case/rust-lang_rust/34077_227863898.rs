
failures:

---- thread_local!_0 stdout ----
    <std macros>:13:1: 13:13 error: macro expansion ignores token `thread_local` and any following
<std macros>:13 thread_local ! ( $ ( $ rest ) * ) ; ) ; (
                ^~~~~~~~~~~~
<anon>:5:1: 10:2 note: caused by the macro expansion here; the usage of `thread_local!` is likely invalid in expression context
<anon>:5 thread_local! {
         ^
<std macros>:16:22: 16:25 error: expected expression, found keyword `pub`
<std macros>:16 $ ( # [ $ attr ] ) * pub static $ name : $ crate :: thread :: LocalKey < $ t >
                                     ^~~
thread 'thread_local!_0' panicked at 'Box<Any>', src/libsyntax/ext/tt/macro_rules.rs:71
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'thread_local!_0' panicked at 'couldn't compile the test', src/librustdoc/test.rs:287


failures:
    thread_local!_0


failures:

---- fmt::write_0 stdout ----
    <anon>:7:5: 7:65 error: unused result which must be used
<anon>:7     fmt::write(&mut output, format_args!("Hello {}!", "world"));
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:1:9: 1:17 note: lint level defined here
<anon>:1 #![deny(warnings)]
                 ^~~~~~~~
error: aborting due to previous error(s)
thread 'fmt::write_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:164
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- fmt::write_1 stdout ----
    <std macros>:2:1: 2:54 error: unused result which must be used
<std macros>:2 $ dst . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:2:1: 2:54 note: in this expansion of write! (defined in <std macros>)
<anon>:1:9: 1:17 note: lint level defined here
<anon>:1 #![deny(warnings)]
                 ^~~~~~~~
error: aborting due to previous error(s)
thread 'fmt::write_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:164


failures:
    fmt::write_0
    fmt::write_1

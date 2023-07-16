

---- memchr::memchr_0 stdout ----
    <anon>:4:9: 4:15 error: unresolved import `memchr::memchr`. Maybe a missing `extern crate memchr`? [E0432]
<anon>:4     use memchr::memchr;
                 ^~~~~~
error: aborting due to previous error
thread 'memchr::memchr_0' panicked at 'Box<Any>', src/libsyntax/diagnostic.rs:240

---- memchr::memrchr_0 stdout ----
    <anon>:4:9: 4:15 error: unresolved import `memchr::memrchr`. Maybe a missing `extern crate memchr`? [E0432]
<anon>:4     use memchr::memrchr;
                 ^~~~~~
error: aborting due to previous error
thread 'memchr::memrchr_0' panicked at 'Box<Any>', src/libsyntax/diagnostic.rs:240


failures:
    memchr::memchr_0
    memchr::memrchr_0

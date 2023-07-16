

failures:

---- path::Path::is_empty_0 stdout ----
    <anon>:8:19: 8:29 error: use of unstable library feature 'os_extras': recently added (see issue #30259)
<anon>:8     assert!(!path.is_empty());
                           ^~~~~~~~~~
<anon>:8:5: 8:31 note: in this expansion of assert! (defined in <std macros>)
<anon>:8:19: 8:29 help: add #![feature(os_extras)] to the crate attributes to enable
error: aborting due to previous error
thread 'path::Path::is_empty_0' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:269


failures:
    path::Path::is_empty_0

test result: [31mFAILED(B[0m. 443 passed; 1 failed; 11 ignored; 0 measured

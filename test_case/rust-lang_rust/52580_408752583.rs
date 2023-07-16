plain
[01:08:22] 
[01:08:22]    Doc-tests core
[01:08:27] 
[01:08:27] running 2102 tests
[01:08:34] .......................................................................F..F.........................
[01:08:42] ...........................F..F.FF..................................................................
[01:09:02] .......................................i............................................................
[01:09:09] ....................................................................................................
[01:09:16] ....................................................................................................
[01:09:23] ....................................................................................................
---
[01:11:10]   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:11:10] 
[01:11:10] thread 'char/methods.rs - char::methods::char::encode_utf16 (line 527)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:11:10] 
[01:11:10] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 551) stdout ----
[01:11:10] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:11:10]  --> char/methods.rs:554:18
[01:11:10]   |
[01:11:10] 6 | let result = '𝕊'.try_encode_utf16(&mut b).unwrap();
[01:11:10]   |
[01:11:10]   |
[01:11:10]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:11:10] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (line 551)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:11:10] 
[01:11:10] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 561) stdout ----
[01:11:10] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 561) stdout ----
[01:11:10] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[01:11:10]  --> char/methods.rs:564:22
[01:11:10]   |
[01:11:10] 6 | assert_eq!(None, '𝕊'.try_encode_utf16(&mut b));
[01:11:10]   |
[01:11:10]   |
[01:11:10]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[01:11:10] 
[01:11:10] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (linnknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
121804 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
118740 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
113396 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
107624 ./src/llvm/test/CodeGen

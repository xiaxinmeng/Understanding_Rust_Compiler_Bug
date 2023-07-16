plain
[00:57:15] 
[00:57:15]    Doc-tests core
[00:57:19] 
[00:57:19] running 2100 tests
[00:57:27] ....................................................................F.F.............................
[00:57:34] .........................F..FF.F....................................................................
[00:57:51] .....................................i..............................................................
[00:57:57] ....................................................................................................
[00:58:04] ....................................................................................................
[00:58:11] ....................................................................................................
---
[00:59:50] 
[00:59:50] failures:
[00:59:50] 
[00:59:50] ---- char/methods.rs - char::methods::char::encode_utf16 (line 515) stdout ----
[00:59:50] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[00:59:50]  --> char/methods.rs:518:18
[00:59:50]   |
[00:59:50] 6 | let result = '𝕊'.try_encode_utf16(&mut b).unwrap();
[00:59:50]   |
[00:59:50]   |
[00:59:50]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[00:59:50] thread 'char/methods.rs - char::methods::char::encode_utf16 (line 515)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[00:59:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:50] 
[00:59:50] ---- char/methods.rs - char::methods::char::encode_utf16 (line 525) stdout ----
[00:59:50] ---- char/methods.rs - char::methods::char::encode_utf16 (line 525) stdout ----
[00:59:50] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[00:59:50]  --> char/methods.rs:528:22
[00:59:50]   |
[00:59:50] 6 | assert_eq!(None, '𝕊'.try_encode_utf16(&mut b));
[00:59:50]   |
[00:59:50]   |
[00:59:50]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[00:59:50] thread 'char/methods.rs - char::methods::char::encode_utf16 (line 525)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[00:59:50] 
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 549) stdout ----
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf16 (line 549) stdout ----
[00:59:50] error[E0599]: no method named `unwrap` found for type `&mut [u16]` in the current scope
[00:59:50]  --> char/methods.rs:552:39
[00:59:50]   |
[00:59:50] 6 | let result = '𝕊'.encode_utf16(&mut b).unwrap();
[00:59:50] 
[00:59:50] thread 'char/methods.rs - char::methods::char::try_encode_utf16 (line 549)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[00:59:50] 
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 455) stdout ----
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 455) stdout ----
[00:59:50] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[00:59:50]  --> char/methods.rs:458:18
[00:59:50]   |
[00:59:50] 6 | let result = 'ß'.try_encode_utf8(&mut b).unwrap();
[00:59:50]   |
[00:59:50]   |
[00:59:50]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[00:59:50] thread 'char/methods.rs - char::methods::char::try_encode_utf8 (line 455)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[00:59:50] 
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 467) stdout ----
[00:59:50] ---- char/methods.rs - char::methods::char::try_encode_utf8 (line 467) stdout ----
[00:59:50] error[E0658]: use of unstable library feature 'try_unicode_encode_char' (see issue #52579)
[00:59:50]  --> char/methods.rs:470:22
[00:59:50]   |
[00:59:50] 6 | assert_eq!(None, 'ß'.try_encode_utf8(&mut b));
[00:59:50]   |
[00:59:50]   |
[00:59:50]   = help: add #![feature(try_unicode_encode_char)] to the crate attributes to enable
[00:59:50] thread 'char/methods.rs -6_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122320 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
121692 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
119000 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
